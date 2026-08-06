use solana_safety_lab::{
    PdaSeedError, ReplayError, ReplayGuard, TransferError, TransferPolicy, TransferRequest,
    VestingError, VestingSchedule, validate_pda_seeds,
};

fn request(amount: u64, spent: u64) -> TransferRequest {
    TransferRequest {
        signer: [1; 32],
        expected_authority: [1; 32],
        source_mint: [2; 32],
        expected_mint: [2; 32],
        destination: [3; 32],
        amount,
        already_spent_today: spent,
    }
}

#[test]
fn transfer_accepts_exact_limits() {
    let policy = TransferPolicy::new(1_000, 5_000).unwrap();
    assert_eq!(policy.validate(&request(1_000, 4_000)), Ok(5_000));
}

#[test]
fn transfer_rejects_authority_mint_and_pause_failures() {
    let policy = TransferPolicy::new(1_000, 5_000).unwrap();
    let mut wrong_authority = request(1, 0);
    wrong_authority.signer = [9; 32];
    assert_eq!(
        policy.validate(&wrong_authority),
        Err(TransferError::AuthorityMismatch)
    );

    let mut wrong_mint = request(1, 0);
    wrong_mint.source_mint = [9; 32];
    assert_eq!(
        policy.validate(&wrong_mint),
        Err(TransferError::MintMismatch)
    );
    assert_eq!(
        policy.paused().validate(&request(1, 0)),
        Err(TransferError::Paused)
    );
}

#[test]
fn transfer_rejects_zero_limits_and_overflow() {
    assert_eq!(TransferPolicy::new(0, 0), Err(TransferError::InvalidPolicy));
    assert_eq!(
        TransferPolicy::new(10, u64::MAX)
            .unwrap()
            .validate(&request(10, u64::MAX)),
        Err(TransferError::ArithmeticOverflow)
    );
}

#[test]
fn vesting_handles_cliff_midpoint_and_end() {
    let schedule = VestingSchedule::new(100, 120, 200, 1_000).unwrap();
    assert_eq!(schedule.vested_at(119), 0);
    assert_eq!(schedule.vested_at(150), 500);
    assert_eq!(schedule.vested_at(200), 1_000);
    assert_eq!(schedule.claimable_at(150, 125), Ok(375));
}

#[test]
fn vesting_rejects_invalid_claims_and_timeline() {
    assert_eq!(
        VestingSchedule::new(10, 9, 20, 1),
        Err(VestingError::InvalidTimeline)
    );
    let schedule = VestingSchedule::new(0, 10, 100, 100).unwrap();
    assert_eq!(
        schedule.claimable_at(20, 30),
        Err(VestingError::ClaimExceedsVested)
    );
}

#[test]
fn seed_policy_checks_shape() {
    assert_eq!(validate_pda_seeds(&[b"vault", &[4; 32]]), Ok(()));
    assert_eq!(validate_pda_seeds(&[]), Err(PdaSeedError::NoSeeds));
    assert!(matches!(
        validate_pda_seeds(&[&[7; 33]]),
        Err(PdaSeedError::SeedTooLong {
            index: 0,
            actual: 33
        })
    ));
}

#[test]
fn replay_window_rejects_duplicates_and_evicts_oldest() {
    let mut guard = ReplayGuard::new(2).unwrap();
    assert_eq!(guard.check_and_record([0; 32]), Err(ReplayError::ZeroNonce));
    guard.check_and_record([1; 32]).unwrap();
    guard.check_and_record([2; 32]).unwrap();
    assert_eq!(guard.check_and_record([1; 32]), Err(ReplayError::Duplicate));
    guard.check_and_record([3; 32]).unwrap();
    assert_eq!(guard.check_and_record([1; 32]), Ok(()));
    assert_eq!(guard.len(), 2);
}
