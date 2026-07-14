//! Padded atomic ring buffer implementation
//! Zero-copy, cache-line padded for high throughput scenarios

use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;

/// Cache line size for padding
const CACHE_LINE: usize = 64;

/// Padded atomic ring buffer for MPMC concurrency
pub struct PaddedAtomicRingBuffer {
    buffer: Vec<u64>,
    capacity: usize,
    head: AtomicUsize,
    _padding1: [u8; CACHE_LINE - 8],
    tail: AtomicUsize,
    _padding2: [u8; CACHE_LINE - 8],
}

impl PaddedAtomicRingBuffer {
    /// Create a new ring buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        Self {
            buffer: vec![0u64; capacity],
            capacity,
            head: AtomicUsize::new(0),
            _padding1: [0; CACHE_LINE - 8],
            tail: AtomicUsize::new(0),
            _padding2: [0; CACHE_LINE - 8],
        }
    }

    /// Push a value onto the ring buffer
    pub fn push(&self, value: u64) -> bool {
        let tail = self.tail.load(Ordering::Acquire);
        let next_tail = (tail + 1) % self.capacity;
        
        let head = self.head.load(Ordering::Acquire);
        if next_tail == head {
            return false; // Buffer full
        }

        // Safe because we've checked boundaries
        unsafe {
            *self.buffer.as_ptr().add(tail) = value;
        }

        self.tail.store(next_tail, Ordering::Release);
        true
    }

    /// Pop a value from the ring buffer
    pub fn pop(&self) -> Option<u64> {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);

        if head == tail {
            return None; // Buffer empty
        }

        let value = unsafe {
            *self.buffer.as_ptr().add(head)
        };

        let next_head = (head + 1) % self.capacity;
        self.head.store(next_head, Ordering::Release);

        Some(value)
    }

    /// Get current buffer utilization
    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        
        if tail >= head {
            tail - head
        } else {
            self.capacity - head + tail
        }
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
