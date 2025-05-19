mod component_state_manager;
mod constants;
mod simulation_stream;
mod simulation_stream_runner;
mod util;

use simulation_stream::TychoSimulation;
use simulation_stream_runner::SimulationStreamRunner;
use ::tycho_simulation::models::Token;
use constants::{TYCHO_API_KEY, TYCHO_ETH_RPC_URL, TYCHO_UNICHAIN_RPC_URL};
use num_bigint::BigUint;
use tracing::info;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tycho_common::models::Chain;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber with pretty formatting
    FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("tycho_cross_chain_arb_bot=debug".parse().unwrap()),
        )
        .with_target(false) // Don't include target in log output
        .with_level(true) // Include log level
        .with_line_number(true) // Include line numbers
        .pretty() // Use pretty formatting
        .init();

    info!("Starting tycho simulation");

    let eth_usdc_token = Token::new(
        "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
        6,
        "USDC",
        BigUint::from(10000u64),
    );
    let eth_usdt_token = Token::new(
        "0xdac17f958d2ee523a2206206994597c13d831ec7",
        6,
        "USDT",
        BigUint::from(10000u64),
    );

    let unichain_usdc_token = Token::new(
        "0x078D782b760474a361dDA0AF3839290b0EF57AD6"
            .to_ascii_lowercase()
            .as_str(),
        6,
        "USDC",
        BigUint::from(10000u64),
    );
    let unichain_usdt_token = Token::new(
        "0x9151434b16b9763660705744891fA906F660EcC5"
            .to_ascii_lowercase()
            .as_str(),
        6,
        "USDT",
        BigUint::from(10000u64),
    );

    info!("Creating eth tycho message processor");
    let eth_tycho_message_processor = TychoSimulation::new(
        TYCHO_ETH_RPC_URL.to_string(),
        TYCHO_API_KEY.to_string(),
        10.0,
        10.0,
        eth_usdc_token,
        eth_usdt_token,
        Chain::Ethereum,
    );

    info!("Creating unichain tycho message processor");
    let unichain_tycho_message_processor = TychoSimulation::new(
        TYCHO_UNICHAIN_RPC_URL.to_string(),
        TYCHO_API_KEY.to_string(),
        10.0,
        10.0,
        unichain_usdc_token,
        unichain_usdt_token,
        Chain::Unichain,
    );

    info!("Creating eth tycho message processor stream");
    let eth_tycho_message_processor_stream =
        eth_tycho_message_processor.create_stream().await.unwrap();
    info!("Creating unichain tycho message processor stream");
    let unichain_tycho_message_processor_stream = unichain_tycho_message_processor
        .create_stream()
        .await
        .unwrap();

    info!("Creating simulation stream runner");
    let simulation_stream_runner = SimulationStreamRunner::new(
        eth_tycho_message_processor_stream,
        unichain_tycho_message_processor_stream,
    );

    info!("Running simulation stream runner");
    let handle = tokio::spawn(simulation_stream_runner.run());

    let _ = handle.await?;

    Ok(())
}
