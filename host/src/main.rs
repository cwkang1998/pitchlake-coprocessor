// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::PRICING_CALCULATOR_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv};
use db_access::DbConnection;
use db_access::queries::get_block_headers_by_block_range;


async fn run_host(start_block: i64, end_block: i64) -> Result<(Option<f64>, Option<f64>, Option<f64>), sqlx::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let db = DbConnection::new().await?;
    let block_headers = get_block_headers_by_block_range(&db.pool, start_block, end_block).await?;

    let env = ExecutorEnv::builder()
        .write(&block_headers)
        .unwrap()
        .build()
        .unwrap();

    let prover = default_prover();

    let prove_info = prover
        .prove(env, PRICING_CALCULATOR_ELF)
        .unwrap();

    let receipt = prove_info.receipt;

    let (volatility, twap, reserve_price): (Option<f64>, Option<f64>, Option<f64>) = receipt.journal.decode().unwrap();

    println!("HOST");
    println!("Volatility: {:?}", volatility);
    println!("TWAP: {:?}", twap);
    println!("Reserve Price: {:?}", reserve_price);

    Ok((volatility, twap, reserve_price))
}


#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    run_host(20000000, 20000170).await?;
    Ok(())
}
