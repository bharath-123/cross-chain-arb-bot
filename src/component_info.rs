
use tracing::info;
use tycho_client::rpc::RPCClient;
use tycho_client::HttpRPCClient;
use std::collections::HashMap;

use tycho_common::dto::{Chain, PaginationParams, ProtocolComponent, ProtocolComponentsRequestBody};

pub struct ComponentMap {
    // component_id -> component
    component_id_map: HashMap<String, ProtocolComponent>,
}

impl ComponentMap {

    pub fn new() -> Self {
        Self {
            component_id_map: HashMap::new(),
        }
    }

    pub fn get_component(&self, component_id: String) -> Option<&ProtocolComponent> {
        self.component_id_map.get(&component_id)
    }

    pub fn get_all_components(&self) -> Vec<&ProtocolComponent> {
        self.component_id_map.values().collect()
    }

    pub fn add_component(&mut self, component: ProtocolComponent) {
        self.component_id_map.insert(component.id.clone(), component);
    }

    pub async fn load_components(chain: Chain, client: &HttpRPCClient, protocol_systems: Vec<String>) -> anyhow::Result<Self> {
        let mut component_id_map = ComponentMap::new();
        info!("Loading protocol components for chain: {}", chain);
        for protocol_system in protocol_systems {
            info!("Loading protocol components for protocol system: {}", protocol_system);
            let mut page_number = 0;
            let page_size = 100;
            loop {
                let components = client.get_protocol_components(&ProtocolComponentsRequestBody {
                    protocol_system: protocol_system.clone(),
                    component_ids: None,
                    tvl_gt: Some(10.0),
                    chain,
                    pagination: PaginationParams::new(page_number, page_size),
                }).await.unwrap();
                if components.protocol_components.is_empty() {
                    break;
                }
                info!("Loaded {} components for protocol system: {}, caching them", components.protocol_components.len(), protocol_system);
                let mut component_count = 0;
                for component in components.protocol_components {
                    info!("Adding component: {}", component.id);
                    component_id_map.add_component(component);
                    component_count += 1;
                }
                info!("Loaded {} components for protocol system: {}", component_count, protocol_system);
                page_number += 1;
            }
        } 

        info!("Loaded {} components", component_id_map.component_id_map.len());
        Ok(component_id_map)
    }
}

pub struct ChainComponentMap {
    // chain -> component_map
    chain_component_map: HashMap<Chain, ComponentMap>,
}

impl ChainComponentMap {
    pub fn new() -> Self {
        Self {
            chain_component_map: HashMap::new(),
        }
    }

    pub fn get_component_map(&self, chain: Chain) -> Option<&ComponentMap> {
        self.chain_component_map.get(&chain)
    }

    pub fn get_component(&self, chain: Chain, component_id: String) -> Option<&ProtocolComponent> {
        self.get_component_map(chain)?.get_component(component_id)
    }

    pub fn add_component(&mut self, chain: Chain, component: ProtocolComponent) {
        let component_map = self.chain_component_map.entry(chain).or_insert(ComponentMap::new());
        component_map.add_component(component);
    }
}