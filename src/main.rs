mod constants;
mod price_reader;
mod tycho_simulation;
mod util;

use ::tycho_simulation::models::Token;
use constants::{TYCHO_API_KEY, TYCHO_ETH_RPC_URL, TYCHO_UNICHAIN_RPC_URL};
use num_bigint::BigUint;
use price_reader::{PriceManager, PriceReader};
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tycho_common::models::Chain;
use tycho_simulation::TychoSimulation;

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber with pretty formatting
    FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("tycho_cross_chain_arb_bot=info".parse().unwrap()),
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
    let (eth_tycho_message_processor, eth_price_receiver) = TychoSimulation::new(
        TYCHO_ETH_RPC_URL.to_string(),
        TYCHO_API_KEY.to_string(),
        10.0,
        10.0,
        eth_usdc_token,
        eth_usdt_token,
        Chain::Ethereum,
    );

    info!("Creating unichain tycho message processor");

    let (unichain_tycho_message_processor, unichain_price_receiver) = TychoSimulation::new(
        TYCHO_UNICHAIN_RPC_URL.to_string(),
        TYCHO_API_KEY.to_string(),
        10.0,
        10.0,
        unichain_usdc_token,
        unichain_usdt_token,
        Chain::Unichain,
    );

    info!("Starting eth tycho message processor");
    let eth_tycho_message_processor_handle = tokio::spawn(eth_tycho_message_processor.run());

    info!("Starting unichain tycho message processor");
    let unichain_tycho_message_processor_handle =
        tokio::spawn(unichain_tycho_message_processor.run());

    let eth_price_reader = PriceReader::new(eth_price_receiver, Chain::Ethereum);
    let unichain_price_reader = PriceReader::new(unichain_price_receiver, Chain::Unichain);

    info!("Starting price manager");
    let price_manager = PriceManager::new(eth_price_reader, unichain_price_reader);
    let price_manager_handle = tokio::spawn(price_manager.run());

    tokio::select! {
        biased;
        _ = eth_tycho_message_processor_handle => {
            error!("Eth tycho message processor panicked");
            return;
        },
        _ = unichain_tycho_message_processor_handle => {
            error!("Unichain tycho message processor panicked");
            return;
        },
        _ = price_manager_handle => {
            error!("Price manager panicked");
            return;
        }
    }
}
