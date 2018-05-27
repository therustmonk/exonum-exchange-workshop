extern crate dotenv;
#[macro_use]
extern crate log;
extern crate exonum;
extern crate cryptoexchange;

use exonum::blockchain::{GenesisConfig, ValidatorKeys};
use exonum::node::{Node, NodeApiConfig, NodeConfig};
use exonum::storage::MemoryDB;
use exonum::node::AllowOrigin;
use cryptoexchange::service;

fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();

    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(vec![validator_keys].into_iter());

    let api_address = "0.0.0.0:8080".parse().unwrap();
    let api_cfg = NodeApiConfig {
        public_api_address: Some(api_address),
        allow_origin: Some(AllowOrigin::Any),
        ..Default::default()
    };

    let peer_address = "0.0.0.0:2000".parse().unwrap();

    NodeConfig {
        listen_address: peer_address,
        peers: vec![],
        service_public_key,
        service_secret_key,
        consensus_public_key,
        consensus_secret_key,
        genesis,
        external_address: None,
        network: Default::default(),
        whitelist: Default::default(),
        api: api_cfg,
        mempool: Default::default(),
        services_configs: Default::default(),
        database: Default::default(),
    }
}

fn main() {
    dotenv::dotenv().ok();
    exonum::helpers::init_logger().unwrap();
    info!("Starting cryptoexchange node");
    let node = Node::new(
        MemoryDB::new(),
        vec![Box::new(service::ExchangeService)],
        node_config(),
    );
    node.run().unwrap();
}
