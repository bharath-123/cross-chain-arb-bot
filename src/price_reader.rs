use anyhow;
use tracing::info;
use tycho_common::models::Chain;
use tycho_simulation::models::Token;

#[derive(Debug, Clone)]
pub struct PriceInfo {
    price: f64,
    block_number: u64,
    token1: Token,
    token2: Token,
    chain: Chain,
    component_id: String,
}

impl PriceInfo {
    pub fn new(price: f64, block_number: u64, chain: Chain, component_id: String, token1: Token, token2: Token) -> Self {
        Self {
            price,
            block_number,
            chain,
            component_id,
            token1,
            token2,
        }
    }
}

pub struct PriceReader {
    price_receiver: tokio::sync::mpsc::Receiver<PriceInfo>,
    chain: Chain,
}

impl PriceReader {
    pub fn new(price_receiver: tokio::sync::mpsc::Receiver<PriceInfo>, chain: Chain) -> Self {
        Self {
            price_receiver,
            chain,
        }
    }

    pub async fn recv(&mut self) -> Option<PriceInfo> {
        self.price_receiver.recv().await
    }
}

pub struct PriceManager {
    latest_block_number: u64,
    chain1_latest_price: Option<f64>,
    chain2_latest_price: Option<f64>,
    chain1_price_reader: PriceReader,
    chain2_price_reader: PriceReader,    
}

impl PriceManager {
    pub fn new(chain1_price_reader: PriceReader, chain2_price_reader: PriceReader) -> Self {
        Self { latest_block_number: 0, chain1_latest_price: None, chain2_latest_price: None, chain1_price_reader, chain2_price_reader }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        loop {
            tokio::select! {
                price = self.chain1_price_reader.recv() => {
                    if let Some(price) = price {
                        info!("{:?} {:?}/{:?} price for block {:?} is: {:?}", self.chain1_price_reader.chain, price.token1.symbol, price.token2.symbol, price.block_number, price.price);
                        self.chain1_latest_price = Some(price.price);

                        if let Some(chain2_price) = self.chain2_latest_price {
                            let price_diff = (price.price - chain2_price).abs();
                            if price_diff > 0.0 {
                                info!("Arbitrage opportunity found: {:?}", price_diff);
                            }
                        }

                        self.latest_block_number = price.block_number;
                    }
                }
                price = self.chain2_price_reader.recv() => {
                    if let Some(price) = price {
                        info!("{:?} {:?}/{:?} price for block {:?} is: {:?}", self.chain2_price_reader.chain, price.token1.symbol, price.token2.symbol, price.block_number, price.price);
                        self.chain2_latest_price = Some(price.price);
                        if let Some(chain1_price) = self.chain1_latest_price {
                            let price_diff = (price.price - chain1_price).abs();
                            if price_diff > 0.0 {
                                info!("Arbitrage opportunity found: {:?}", price_diff);
                            }
                        }

                        self.latest_block_number = price.block_number;
                    }
                }
            }
        }
    }
}
