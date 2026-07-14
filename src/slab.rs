//! Lock-free slab allocator

use core::sync::atomic::{AtomicUsize, Ordering};

/// Lock-free slab allocator for object pools
pub struct LockFreeSlab {
    capacity: usize,
    free_list: AtomicUsize,
}

impl LockFreeSlab {
    /// Create a new slab allocator
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            free_list: AtomicUsize::new(0),
        }
    }

    /// Allocate a slot
    pub fn allocate(&self) -> Option<usize> {
        let mut slot = self.free_list.load(Ordering::Acquire);
        
        if slot >= self.capacity {
            return None;
        }

        loop {
            match self.free_list.compare_exchange(
                slot,
                slot + 1,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => return Some(slot),
                Err(actual) => slot = actual,
            }
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
