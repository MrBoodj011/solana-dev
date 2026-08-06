use solana_safety_lab::{ReplayGuard, TransferPolicy, TransferRequest, VestingSchedule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let authority = [7; 32];
    let mint = [9; 32];
    let next_total = TransferPolicy::new(1_000, 5_000)?.validate(&TransferRequest {
        signer: authority,
        expected_authority: authority,
        source_mint: mint,
        expected_mint: mint,
        destination: [3; 32],
        amount: 750,
        already_spent_today: 1_000,
    })?;

    let schedule = VestingSchedule::new(0, 100, 1_000, 10_000)?;
    let mut replay = ReplayGuard::new(128)?;
    replay.check_and_record([42; 32])?;

    println!("new daily total: {next_total}");
    println!("vested at timestamp 550: {}", schedule.vested_at(550));
    println!("tracked nonces: {}", replay.len());
    Ok(())
}
