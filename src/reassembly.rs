//! Zero-copy message reassembly

/// Zero-copy message reassembly for distributed systems
pub struct ZeroCopyReassembly;

impl ZeroCopyReassembly {
    /// Create new reassembly context
    pub fn new() -> Self {
        Self
    }

    /// Reassemble fragments without copying
    pub fn reassemble(fragments: &[&[u8]]) -> usize {
        fragments.iter().map(|f| f.len()).sum()
    }
}

impl Default for ZeroCopyReassembly {
    fn default() -> Self {
        Self::new()
    }
}
