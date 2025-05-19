use core::task::Context;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::Poll;

use futures::{Stream, StreamExt};
use tycho_client::feed::component_tracker::ComponentFilter;
use tycho_common::models::Chain;
use tycho_common::Bytes;
use tycho_simulation::evm::decoder::StreamDecodeError;
use tycho_simulation::protocol::models::BlockUpdate;
use tycho_simulation::{evm::stream::ProtocolStreamBuilder, models::Token};

use crate::util::register_exchanges;

pub struct TychoSimulationStreamInfo {
    stream: Pin<Box<dyn Stream<Item = Result<BlockUpdate, StreamDecodeError>> + Send + Sync>>,
    token1: Token,
    token2: Token,
    chain: Chain,
}

impl TychoSimulationStreamInfo {
    pub fn new(
        stream: Box<dyn Stream<Item = Result<BlockUpdate, StreamDecodeError>> + Send + Sync>,
        token1: Token,
        token2: Token,
        chain: Chain,
    ) -> Self {
        Self {
            stream: Box::into_pin(stream),
            token1,
            token2,
            chain,
        }
    }

    pub fn get_token1(&self) -> &Token {
        &self.token1
    }

    pub fn get_token2(&self) -> &Token {
        &self.token2
    }

    pub fn get_chain(&self) -> &Chain {
        &self.chain
    }
}

impl Stream for TychoSimulationStreamInfo {
    type Item = Result<BlockUpdate, StreamDecodeError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.stream.poll_next_unpin(cx)
    }
}

pub struct TychoSimulation {
    pub url: String,
    pub api_key: String,
    pub remove_tvl_threshold: f64,
    pub add_tvl_threshold: f64,
    pub token_map: HashMap<Bytes, Token>,
    pub token1: Token,
    pub token2: Token,
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
    ) -> Self {
        let mut token_map = HashMap::new();
        token_map.insert(token1.address.clone(), token1.clone());
        token_map.insert(token2.address.clone(), token2.clone());

        Self {
            url,
            api_key,
            remove_tvl_threshold,
            add_tvl_threshold,
            token_map,
            token1,
            token2,
            chain,
        }
    }

    pub async fn create_stream(self) -> anyhow::Result<TychoSimulationStreamInfo> {
        let tvl_filter =
            ComponentFilter::with_tvl_range(self.remove_tvl_threshold, self.add_tvl_threshold);
        let protocol_stream = register_exchanges(
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

        return Ok(TychoSimulationStreamInfo::new(
            Box::new(protocol_stream),
            self.token1,
            self.token2,
            self.chain,
        ));
    }
}
