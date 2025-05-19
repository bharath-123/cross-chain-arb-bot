use std::collections::HashMap;
use tycho_common::models::Chain;

#[derive(Debug, Clone)]
pub struct ComponentState {
    spot_price: f64,
    id: String,
}

impl ComponentState {
    pub fn new(spot_price: f64, id: String) -> Self {
        Self { spot_price, id }
    }

    pub fn get_spot_price(&self) -> f64 {
        self.spot_price
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }
}

#[derive(Debug)]
pub struct ComponentStateManager {
    // (chain, component_id) -> spot_price
    latest_state: HashMap<(Chain, String), ComponentState>,
}

impl ComponentStateManager {
    pub fn new() -> Self {
        Self {
            latest_state: HashMap::new(),
        }
    }

    pub fn update_component_state(
        &mut self,
        chain: Chain,
        component_id: String,
        state: ComponentState,
    ) {
        self.latest_state.insert((chain, component_id), state);
    }

    pub fn get_component_state(
        &self,
        chain: Chain,
        component_id: String,
    ) -> Option<&ComponentState> {
        self.latest_state.get(&(chain, component_id))
    }

    pub fn get_all_component_state_for_chain(&self, chain: Chain) -> Vec<&ComponentState> {
        self.latest_state
            .iter()
            .filter(|(k, _)| k.0 == chain)
            .map(|(_, v)| v)
            .collect()
    }
}
