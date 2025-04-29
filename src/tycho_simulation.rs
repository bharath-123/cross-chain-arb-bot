use std::collections::{HashMap, HashSet};

use futures::StreamExt;
use tokio::sync::mpsc;
use tracing::info;
use tycho_client::feed::component_tracker::ComponentFilter;
use tycho_common::models::Chain;
use tycho_common::Bytes;
use tycho_simulation::{evm::stream::ProtocolStreamBuilder, models::Token};

use crate::price_reader::PriceInfo;
use crate::util::register_exchanges;
pub struct TychoSimulation {
    pub url: String,
    pub api_key: String,
    pub remove_tvl_threshold: f64,
    pub add_tvl_threshold: f64,
    pub token_map: HashMap<Bytes, Token>,
    pub token1: Token,
    pub token2: Token,
    pub relevant_pool_ids: HashSet<String>,
    pub price_sender: tokio::sync::mpsc::Sender<PriceInfo>,
    pub chain: Chain,
}

impl TychoSimulation {
    pub fn new(
        url: String,
        api_key: String,
        remove_tvl_threshold: f64,
        add_tvl_threshold: f64,
        token1: Token,
        token2: Token,
        chain: Chain,
    ) -> (Self, tokio::sync::mpsc::Receiver<PriceInfo>) {
        let mut token_map = HashMap::new();
        token_map.insert(token1.address.clone(), token1.clone());
        token_map.insert(token2.address.clone(), token2.clone());

        let relevant_pool_ids = HashSet::new();

        let (price_sender, price_receiver) = mpsc::channel(100);
        (
            Self {
                url,
                api_key,
                remove_tvl_threshold,
                add_tvl_threshold,
                token_map,
                token1,
                token2,
                relevant_pool_ids,
                price_sender,
                chain,
            },
            price_receiver,
        )
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        // TODO - we should connect in the `new` method. The `run` method should just be a loop consuming the
        // protocol stream.
        let tvl_filter =
            ComponentFilter::with_tvl_range(self.remove_tvl_threshold, self.add_tvl_threshold);
        let mut protocol_stream = register_exchanges(
            ProtocolStreamBuilder::new(&self.url, self.chain),
            &self.chain,
            tvl_filter,
        )
        .auth_key(Some(self.api_key.to_string()))
        .skip_state_decode_failures(true)
        .set_tokens(self.token_map)
        .await
        .build()
        .await
        .expect("Failed building protocol stream");

        info!("Starting to listen to protocol stream for {}", self.chain);

        // Loop through block updates
        while let Some(msg) = protocol_stream.next().await {
            if let Ok(msg) = msg {
                info!("Checking {:?} block {:?}", self.chain, msg.block_number);
                for (k, v) in msg.new_pairs {
                    // Check if the pair contains both USDC and USDT tokens
                    let token1 = v
                        .tokens
                        .iter()
                        .find(|token| token.symbol.eq(&self.token1.symbol));

                    let token2 = v
                        .tokens
                        .iter()
                        .find(|token| token.symbol.eq(&self.token2.symbol));

                    // If both tokens are present, store the pool ID
                    if token1.is_some() && token2.is_some() {
                        info!("Caching pool id: {:?}", k);
                        self.relevant_pool_ids.insert(k.clone());
                    }
                }
                for (k, _) in msg.removed_pairs {
                    if self.relevant_pool_ids.remove(&k) {
                        info!("Removing pool id: {:?}", k);
                    }
                }
                for pool_id in self.relevant_pool_ids.iter() {
                    if let Some(pool) = msg.states.get(pool_id) {
                        let price = pool.spot_price(&self.token1, &self.token2)?;        

                        self.price_sender
                            .send(PriceInfo::new(price, msg.block_number, self.chain, pool_id.clone(), self.token1.clone(), self.token2.clone()))
                            .await?;
                    }
                }
            }
        }
        return anyhow::Result::Ok(());
    }
}
