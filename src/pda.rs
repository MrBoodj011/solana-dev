use core::fmt;

/// Maximum number of seeds accepted by the strict policy.
pub const MAX_PDA_SEEDS: usize = 16;
/// Maximum byte length accepted for an individual seed.
pub const MAX_PDA_SEED_LEN: usize = 32;

/// Validates seed structure before it is passed to a Solana SDK derivation API.
pub fn validate_pda_seeds(seeds: &[&[u8]]) -> Result<(), PdaSeedError> {
    if seeds.is_empty() {
        return Err(PdaSeedError::NoSeeds);
    }
    if seeds.len() > MAX_PDA_SEEDS {
        return Err(PdaSeedError::TooManySeeds {
            actual: seeds.len(),
        });
    }
    for (index, seed) in seeds.iter().enumerate() {
        if seed.is_empty() {
            return Err(PdaSeedError::EmptySeed { index });
        }
        if seed.len() > MAX_PDA_SEED_LEN {
            return Err(PdaSeedError::SeedTooLong {
                index,
                actual: seed.len(),
            });
        }
    }
    Ok(())
}

/// Why a PDA seed set was rejected by the strict application policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdaSeedError {
    /// No domain or subject seed was provided.
    NoSeeds,
    /// A seed is empty.
    EmptySeed {
        /// Zero-based seed position.
        index: usize,
    },
    /// Too many seeds were provided.
    TooManySeeds {
        /// Number of supplied seeds.
        actual: usize,
    },
    /// An individual seed is too long.
    SeedTooLong {
        /// Zero-based seed position.
        index: usize,
        /// Supplied byte length.
        actual: usize,
    },
}

impl fmt::Display for PdaSeedError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSeeds => formatter.write_str("at least one PDA seed is required"),
            Self::EmptySeed { index } => write!(formatter, "PDA seed {index} is empty"),
            Self::TooManySeeds { actual } => {
                write!(formatter, "PDA seed count {actual} exceeds {MAX_PDA_SEEDS}")
            }
            Self::SeedTooLong { index, actual } => write!(
                formatter,
                "PDA seed {index} has {actual} bytes; maximum is {MAX_PDA_SEED_LEN}"
            ),
        }
    }
}

impl std::error::Error for PdaSeedError {}
