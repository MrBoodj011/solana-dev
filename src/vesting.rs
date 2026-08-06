use core::fmt;

/// A cliff followed by linear vesting between `start_ts` and `end_ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VestingSchedule {
    start_ts: u64,
    cliff_ts: u64,
    end_ts: u64,
    total_amount: u64,
}

impl VestingSchedule {
    /// Builds a validated vesting schedule.
    pub fn new(
        start_ts: u64,
        cliff_ts: u64,
        end_ts: u64,
        total_amount: u64,
    ) -> Result<Self, VestingError> {
        if total_amount == 0 {
            return Err(VestingError::ZeroAllocation);
        }
        if start_ts > cliff_ts || cliff_ts > end_ts || start_ts == end_ts {
            return Err(VestingError::InvalidTimeline);
        }
        Ok(Self {
            start_ts,
            cliff_ts,
            end_ts,
            total_amount,
        })
    }

    /// Returns the total amount vested at `now_ts`.
    #[must_use]
    pub fn vested_at(&self, now_ts: u64) -> u64 {
        if now_ts < self.cliff_ts {
            return 0;
        }
        if now_ts >= self.end_ts {
            return self.total_amount;
        }
        let elapsed = now_ts.saturating_sub(self.start_ts) as u128;
        let duration = (self.end_ts - self.start_ts) as u128;
        ((self.total_amount as u128 * elapsed) / duration) as u64
    }

    /// Returns the amount currently claimable after subtracting prior claims.
    pub fn claimable_at(&self, now_ts: u64, claimed: u64) -> Result<u64, VestingError> {
        if claimed > self.total_amount {
            return Err(VestingError::ClaimExceedsAllocation);
        }
        self.vested_at(now_ts)
            .checked_sub(claimed)
            .ok_or(VestingError::ClaimExceedsVested)
    }

    /// Returns the schedule's total allocation.
    #[must_use]
    pub const fn total_amount(&self) -> u64 {
        self.total_amount
    }
}

/// Why vesting construction or claim calculation failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VestingError {
    /// The allocation is zero.
    ZeroAllocation,
    /// Start, cliff, and end timestamps are not ordered correctly.
    InvalidTimeline,
    /// Prior claims exceed the full allocation.
    ClaimExceedsAllocation,
    /// Prior claims exceed the amount vested at the requested time.
    ClaimExceedsVested,
}

impl fmt::Display for VestingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ZeroAllocation => "vesting allocation must be greater than zero",
            Self::InvalidTimeline => "vesting timestamps are invalid",
            Self::ClaimExceedsAllocation => "claimed amount exceeds the allocation",
            Self::ClaimExceedsVested => "claimed amount exceeds the vested amount",
        })
    }
}

impl std::error::Error for VestingError {}
