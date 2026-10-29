//! Deterministic bounded chunk request queue for future async workers.
use rustcraft_core::ChunkPos;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct ChunkRequests {
    capacity: usize,
    pending: VecDeque<ChunkPos>,
    seen: HashSet<ChunkPos>,
}

impl ChunkRequests {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            pending: VecDeque::new(),
            seen: HashSet::new(),
        }
    }

    pub fn enqueue(&mut self, position: ChunkPos) -> bool {
        if self.seen.contains(&position) || self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(position);
        self.seen.insert(position);
        true
    }

    pub fn pop(&mut self) -> Option<ChunkPos> {
        let position = self.pending.pop_front()?;
        self.seen.remove(&position);
        Some(position)
    }

    pub fn len(&self) -> usize {
        self.pending.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_is_bounded_and_deduplicated() {
        let a = ChunkPos { x: 0, y: 0, z: 0 };
        let b = ChunkPos { x: 1, y: 0, z: 0 };
        let mut queue = ChunkRequests::new(1);
        assert!(queue.enqueue(a));
        assert!(!queue.enqueue(a));
        assert!(!queue.enqueue(b));
        assert_eq!(queue.pop(), Some(a));
        assert!(queue.enqueue(b));
        assert_eq!(queue.len(), 1);
    }
}
