use crate::Address;
use core::fmt;

/// A transfer request evaluated by [`TransferPolicy`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferRequest {
    /// Address that signed the instruction.
    pub signer: Address,
    /// Authority required by the application.
    pub expected_authority: Address,
    /// Mint attached to the source account.
    pub source_mint: Address,
    /// Mint permitted by the application.
    pub expected_mint: Address,
    /// Destination token-account address.
    pub destination: Address,
    /// Token amount in base units.
    pub amount: u64,
    /// Amount already spent in the current application-defined window.
    pub already_spent_today: u64,
}

/// Deterministic transfer limits for one application domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TransferPolicy {
    max_per_transfer: u64,
    max_per_day: u64,
    paused: bool,
}

impl TransferPolicy {
    /// Creates an active policy. Both limits must be non-zero and the daily
    /// limit must be at least as large as the per-transfer limit.
    pub const fn new(max_per_transfer: u64, max_per_day: u64) -> Result<Self, TransferError> {
        if max_per_transfer == 0 || max_per_day < max_per_transfer {
            return Err(TransferError::InvalidPolicy);
        }
        Ok(Self {
            max_per_transfer,
            max_per_day,
            paused: false,
        })
    }

    /// Returns a copy that rejects every transfer.
    #[must_use]
    pub const fn paused(mut self) -> Self {
        self.paused = true;
        self
    }

    /// Validates a request and returns its new daily total.
    pub fn validate(&self, request: &TransferRequest) -> Result<u64, TransferError> {
        if self.paused {
            return Err(TransferError::Paused);
        }
        if request.signer != request.expected_authority {
            return Err(TransferError::AuthorityMismatch);
        }
        if request.source_mint != request.expected_mint {
            return Err(TransferError::MintMismatch);
        }
        if request.destination == [0; 32] {
            return Err(TransferError::ZeroDestination);
        }
        if request.amount == 0 {
            return Err(TransferError::ZeroAmount);
        }
        if request.amount > self.max_per_transfer {
            return Err(TransferError::PerTransferLimit);
        }
        let next_total = request
            .already_spent_today
            .checked_add(request.amount)
            .ok_or(TransferError::ArithmeticOverflow)?;
        if next_total > self.max_per_day {
            return Err(TransferError::DailyLimit);
        }
        Ok(next_total)
    }
}

/// Why a transfer request was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferError {
    /// The policy limits are internally inconsistent.
    InvalidPolicy,
    /// Transfers are paused.
    Paused,
    /// The signer is not the configured authority.
    AuthorityMismatch,
    /// The source account uses an unexpected mint.
    MintMismatch,
    /// The destination is the all-zero address.
    ZeroDestination,
    /// A zero-value transfer was requested.
    ZeroAmount,
    /// The single-transfer limit was exceeded.
    PerTransferLimit,
    /// The daily application limit was exceeded.
    DailyLimit,
    /// Adding the request to prior usage overflowed `u64`.
    ArithmeticOverflow,
}

impl fmt::Display for TransferError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidPolicy => "transfer policy is invalid",
            Self::Paused => "transfers are paused",
            Self::AuthorityMismatch => "transfer authority does not match",
            Self::MintMismatch => "source mint does not match",
            Self::ZeroDestination => "destination cannot be the zero address",
            Self::ZeroAmount => "transfer amount must be greater than zero",
            Self::PerTransferLimit => "per-transfer limit exceeded",
            Self::DailyLimit => "daily transfer limit exceeded",
            Self::ArithmeticOverflow => "transfer total overflowed",
        })
    }
}

impl std::error::Error for TransferError {}
