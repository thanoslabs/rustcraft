//! Bounded background terrain generation with non-blocking result collection.
use crate::{ChunkSection, terrain::generate_section};
use rustcraft_core::ChunkPos;
use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
    },
    thread::{self, JoinHandle},
};

pub struct GeneratedChunk {
    pub position: ChunkPos,
    pub section: ChunkSection,
}

pub struct ChunkWorkers {
    sender: Option<SyncSender<ChunkPos>>,
    receiver: Receiver<GeneratedChunk>,
    threads: Vec<JoinHandle<()>>,
}

impl ChunkWorkers {
    pub fn new(seed: u64, worker_count: usize, queue_capacity: usize) -> Self {
        let (sender, requests) = mpsc::sync_channel::<ChunkPos>(queue_capacity);
        let (results, receiver) = mpsc::sync_channel::<GeneratedChunk>(queue_capacity);
        let requests = Arc::new(Mutex::new(requests));
        let mut threads = Vec::new();
        for _ in 0..worker_count.max(1) {
            let requests = Arc::clone(&requests);
            let results = results.clone();
            threads.push(thread::spawn(move || {
                loop {
                    let request = {
                        let Ok(receiver) = requests.lock() else { break };
                        receiver.recv()
                    };
                    let Ok(position) = request else { break };
                    let section = generate_section(seed, position);
                    if results.send(GeneratedChunk { position, section }).is_err() {
                        break;
                    }
                }
            }));
        }
        Self {
            sender: Some(sender),
            receiver,
            threads,
        }
    }

    /// Returns false when the bounded work queue is full or disconnected.
    pub fn try_submit(&self, position: ChunkPos) -> bool {
        self.sender
            .as_ref()
            .is_some_and(|sender| match sender.try_send(position) {
                Ok(()) => true,
                Err(TrySendError::Full(_) | TrySendError::Disconnected(_)) => false,
            })
    }

    pub fn try_receive(&self) -> Option<GeneratedChunk> {
        match self.receiver.try_recv() {
            Ok(chunk) => Some(chunk),
            Err(TryRecvError::Empty | TryRecvError::Disconnected) => None,
        }
    }
}

impl Drop for ChunkWorkers {
    fn drop(&mut self) {
        self.sender.take();
        // Do not block the rendering thread waiting for background workers.
        self.threads.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn generates_sections_off_thread() {
        let workers = ChunkWorkers::new(42, 2, 4);
        let position = ChunkPos { x: 0, y: 0, z: 0 };
        assert!(workers.try_submit(position));
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(result) = workers.try_receive() {
                assert_eq!(result.position, position);
                assert!(result.section.non_air_count() > 0);
                break;
            }
            assert!(Instant::now() < deadline, "worker timed out");
            thread::sleep(Duration::from_millis(5));
        }
    }
}
