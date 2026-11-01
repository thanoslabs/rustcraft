//! Bounded, distance-prioritized chunk lifecycle for streaming worlds.
use rustcraft_core::ChunkPos;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Requested,
    Generating,
    Ready,
}

#[derive(Debug)]
pub struct ChunkScheduler {
    capacity: usize,
    states: HashMap<ChunkPos, ChunkState>,
    center: ChunkPos,
    radius: i64,
}

impl ChunkScheduler {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            states: HashMap::new(),
            center: ChunkPos { x: 0, y: 0, z: 0 },
            radius: 0,
        }
    }

    /// Refresh the desired horizontal view. In-flight work can finish safely,
    /// but out-of-range results will not be retained.
    pub fn update_view(&mut self, center: ChunkPos, radius: i64) {
        self.center = center;
        self.radius = radius.max(0);
        self.states.retain(|pos, _| {
            (i128::from(pos.x) - i128::from(center.x)).abs() <= i128::from(self.radius)
                && (i128::from(pos.z) - i128::from(center.z)).abs() <= i128::from(self.radius)
                && pos.y == center.y
        });
        let mut candidates = Vec::new();
        for z in -self.radius..=self.radius {
            for x in -self.radius..=self.radius {
                if let (Some(cx), Some(cz)) = (center.x.checked_add(x), center.z.checked_add(z)) {
                    let pos = ChunkPos {
                        x: cx,
                        y: center.y,
                        z: cz,
                    };
                    if !self.states.contains_key(&pos) {
                        candidates.push(pos);
                    }
                }
            }
        }
        candidates.sort_by_key(|p| (self.distance_squared(*p), p.x, p.z));
        for pos in candidates {
            if self.states.len() >= self.capacity {
                break;
            }
            self.states.insert(pos, ChunkState::Requested);
        }
    }

    fn distance_squared(&self, pos: ChunkPos) -> i128 {
        let dx = i128::from(pos.x) - i128::from(self.center.x);
        let dz = i128::from(pos.z) - i128::from(self.center.z);
        dx * dx + dz * dz
    }

    pub fn next_request(&mut self) -> Option<ChunkPos> {
        let pos = self
            .states
            .iter()
            .filter(|(_, state)| **state == ChunkState::Requested)
            .map(|(pos, _)| *pos)
            .min_by_key(|p| (self.distance_squared(*p), p.x, p.z))?;
        self.states.insert(pos, ChunkState::Generating);
        Some(pos)
    }

    /// Requeue a failed or rejected submission so the next frame can retry it.
    pub fn retry(&mut self, pos: ChunkPos) -> bool {
        if self.states.get(&pos) != Some(&ChunkState::Generating) {
            return false;
        }
        self.states.insert(pos, ChunkState::Requested);
        true
    }

    pub fn complete(&mut self, pos: ChunkPos) -> bool {
        if self.states.get(&pos) != Some(&ChunkState::Generating) {
            return false;
        }
        self.states.insert(pos, ChunkState::Ready);
        true
    }

    pub fn state(&self, pos: ChunkPos) -> Option<ChunkState> {
        self.states.get(&pos).copied()
    }

    pub fn loaded_positions(&self) -> HashSet<ChunkPos> {
        self.states
            .iter()
            .filter(|(_, state)| **state == ChunkState::Ready)
            .map(|(pos, _)| *pos)
            .collect()
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(x: i64, z: i64) -> ChunkPos {
        ChunkPos { x, y: 0, z }
    }

    #[test]
    fn nearest_first_and_deduplicated() {
        let mut scheduler = ChunkScheduler::new(9);
        scheduler.update_view(pos(0, 0), 1);
        scheduler.update_view(pos(0, 0), 1);
        assert_eq!(scheduler.len(), 9);
        assert_eq!(scheduler.next_request(), Some(pos(0, 0)));
        assert_eq!(scheduler.state(pos(0, 0)), Some(ChunkState::Generating));
        assert!(scheduler.complete(pos(0, 0)));
        assert_eq!(scheduler.state(pos(0, 0)), Some(ChunkState::Ready));
    }

    #[test]
    fn moving_view_discards_stale_results() {
        let mut scheduler = ChunkScheduler::new(9);
        scheduler.update_view(pos(0, 0), 1);
        let old = scheduler.next_request().unwrap();
        scheduler.update_view(pos(100, 100), 1);
        assert!(!scheduler.complete(old));
        assert_eq!(scheduler.next_request(), Some(pos(100, 100)));
        assert!(scheduler.len() <= 9);
    }

    #[test]
    fn failed_submission_can_be_retried() {
        let mut scheduler = ChunkScheduler::new(1);
        scheduler.update_view(pos(0, 0), 0);
        let requested = scheduler.next_request().unwrap();
        assert!(scheduler.retry(requested));
        assert_eq!(scheduler.state(requested), Some(ChunkState::Requested));
        assert_eq!(scheduler.next_request(), Some(requested));
        assert!(!scheduler.retry(pos(4, 4)));
    }

    #[test]
    fn extreme_coordinates_do_not_overflow() {
        let mut scheduler = ChunkScheduler::new(2);
        scheduler.update_view(pos(i64::MIN, i64::MIN), 1);
        scheduler.update_view(pos(i64::MAX, i64::MAX), 1);
        assert!(scheduler.len() <= 2);
    }

    #[test]
    fn respects_capacity() {
        let mut scheduler = ChunkScheduler::new(3);
        scheduler.update_view(pos(0, 0), 12);
        assert_eq!(scheduler.len(), 3);
    }
}
