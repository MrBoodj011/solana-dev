//! Pure, dependency-free guardrails for security-sensitive Solana program logic.
//!
//! The crate deliberately avoids RPC, signing, account deserialization, and I/O.
//! Integrators can map these deterministic decisions into Anchor or native Solana
//! instruction handlers and test the account layer separately.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod pda;
mod replay;
mod transfer;
mod vesting;

/// A raw 32-byte address used by the pure policy layer.
///
/// Convert to and from your Solana SDK `Pubkey` at the integration boundary.
pub type Address = [u8; 32];

pub use pda::{MAX_PDA_SEED_LEN, MAX_PDA_SEEDS, PdaSeedError, validate_pda_seeds};
pub use replay::{ReplayError, ReplayGuard};
pub use transfer::{TransferError, TransferPolicy, TransferRequest};
pub use vesting::{VestingError, VestingSchedule};
