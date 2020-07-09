use serde::{Deserialize, Serialize};

use crate::{
    ApiServerConfig, ContractRuntimeConfig, GossipTableConfig, SmallNetworkConfig, StorageConfig,
    ROOT_VALIDATOR_LISTENING_PORT,
};

/// Root configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// Network configuration for the validator-only network.
    pub validator_net: SmallNetworkConfig,
    /// Network configuration for the HTTP API.
    pub http_server: ApiServerConfig,
    /// On-disk storage configuration.
    pub storage: StorageConfig,
    /// Gossip protocol configuration.
    pub gossip: GossipTableConfig,
    /// Contract runtime configuration.
    pub contract_runtime: ContractRuntimeConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            validator_net: SmallNetworkConfig::default_on_port(ROOT_VALIDATOR_LISTENING_PORT),
            http_server: ApiServerConfig::default(),
            storage: StorageConfig::default(),
            gossip: GossipTableConfig::default(),
            contract_runtime: ContractRuntimeConfig::default(),
        }
    }
}