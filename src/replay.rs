use core::fmt;
use std::collections::VecDeque;

/// A bounded, deterministic replay window for 32-byte request nonces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayGuard {
    capacity: usize,
    recent: VecDeque<[u8; 32]>,
}

impl ReplayGuard {
    /// Creates a replay window. Capacity must be greater than zero.
    pub fn new(capacity: usize) -> Result<Self, ReplayError> {
        if capacity == 0 {
            return Err(ReplayError::ZeroCapacity);
        }
        Ok(Self {
            capacity,
            recent: VecDeque::with_capacity(capacity),
        })
    }

    /// Rejects an invalid or recently observed nonce, otherwise records it.
    pub fn check_and_record(&mut self, nonce: [u8; 32]) -> Result<(), ReplayError> {
        if nonce == [0; 32] {
            return Err(ReplayError::ZeroNonce);
        }
        if self.recent.contains(&nonce) {
            return Err(ReplayError::Duplicate);
        }
        if self.recent.len() == self.capacity {
            self.recent.pop_front();
        }
        self.recent.push_back(nonce);
        Ok(())
    }

    /// Number of nonces currently retained.
    #[must_use]
    pub fn len(&self) -> usize {
        self.recent.len()
    }

    /// Whether the replay window is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.recent.is_empty()
    }
}

/// Why replay protection rejected an operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayError {
    /// Replay history cannot have zero capacity.
    ZeroCapacity,
    /// The all-zero nonce is reserved and invalid.
    ZeroNonce,
    /// The nonce is already present in the active replay window.
    Duplicate,
}

impl fmt::Display for ReplayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ZeroCapacity => "replay capacity must be greater than zero",
            Self::ZeroNonce => "zero nonce is not allowed",
            Self::Duplicate => "nonce has already been observed",
        })
    }
}

impl std::error::Error for ReplayError {}
