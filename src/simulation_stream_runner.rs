use std::collections::HashSet;

use futures::StreamExt;
use tracing::info;
use tycho_common::models::Chain;
use tycho_simulation::{models::Token, protocol::models::BlockUpdate};

use crate::{
    component_state_manager::{ComponentState, ComponentStateManager},
    simulation_stream::TychoSimulationStreamInfo,
};
pub struct SimulationStreamRunner {
    chain1_stream: TychoSimulationStreamInfo,
    chain2_stream: TychoSimulationStreamInfo,
    relevant_pool_ids: HashSet<String>,
    component_state_manager: ComponentStateManager,
}

impl SimulationStreamRunner {
    pub fn new(
        chain1_stream: TychoSimulationStreamInfo,
        chain2_stream: TychoSimulationStreamInfo,
    ) -> Self {
        Self {
            chain1_stream,
            chain2_stream,
            relevant_pool_ids: HashSet::new(),
            component_state_manager: ComponentStateManager::new(),
        }
    }

    pub async fn run(mut self) -> anyhow::Result<()> {
        let chain1 = self.chain1_stream.get_chain().clone();
        let chain2 = self.chain2_stream.get_chain().clone();

        loop {
            tokio::select! {
                Some(block_update) = self.chain1_stream.next() => {
                    if let Ok(block_update) = block_update {
                        process_block_update(&mut self.component_state_manager, &mut self.relevant_pool_ids, self.chain1_stream.get_token1(), self.chain1_stream.get_token2(), &chain1, &chain2, block_update)?;
                    }
                }
                Some(block_update) = self.chain2_stream.next() => {
                    if let Ok(block_update) = block_update {
                        process_block_update(&mut self.component_state_manager, &mut self.relevant_pool_ids, self.chain2_stream.get_token1(), self.chain2_stream.get_token2(), &chain2, &chain1, block_update)?;
                    }
                }
            }
        }
    }
}

fn process_block_update(
    component_state_manager: &mut ComponentStateManager,
    relevant_pool_ids: &mut HashSet<String>,
    token1: &Token,
    token2: &Token,
    chain: &Chain,
    other_chain: &Chain,
    block_update: BlockUpdate,
) -> anyhow::Result<()> {
    info!("Checking {:?} block {:?}", chain, block_update.block_number);
    for (k, v) in block_update.new_pairs {
        // Check if the pair contains both USDC and USDT tokens
        let token1 = v
            .tokens
            .iter()
            .find(|token| token.symbol.eq(&token1.symbol));

        let token2 = v
            .tokens
            .iter()
            .find(|token| token.symbol.eq(&token2.symbol));

        // If both tokens are present, store the pool ID
        if token1.is_some() && token2.is_some() {
            info!("Caching pool id: {:?}", k);
            relevant_pool_ids.insert(k.clone());
        }
    }
    for (k, _) in block_update.removed_pairs {
        if relevant_pool_ids.remove(&k) {
            info!("Removing pool id: {:?}", k);
        }
    }
    for pool_id in relevant_pool_ids.iter() {
        if let Some(pool) = block_update.states.get(pool_id) {
            let price = pool.spot_price(&token1, &token2)?;
            info!(
                "Price for {:?}/{:?} on chain:{:?} is {:?}",
                token1.symbol, token2.symbol, chain, price
            );
            component_state_manager.update_component_state(
                chain.clone(),
                pool_id.clone(),
                ComponentState::new(price, pool_id.clone()),
            );

            info!("Checking for arb opportunities on {:?}", other_chain);
            // check if we have an arb opportunity against the other chain
            let other_chain_components =
                component_state_manager.get_all_component_state_for_chain(*other_chain);
            for component in other_chain_components {
                let price_diff = (price - component.get_spot_price()).abs();
                if price_diff > 0.0 {
                    info!(
                        "Arb opportunity found on {:?}/{:?} on chain:{:?} with price diff {:?}",
                        token1.symbol, token2.symbol, other_chain, price_diff
                    );
                    // we need to compute the amount out and execute the arb
                }
            }
        }
    }
    Ok(())
}
