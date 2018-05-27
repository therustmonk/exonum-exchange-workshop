#[macro_use]
extern crate log;
extern crate dotenv;
extern crate exonum;

fn node_config() -> NodeConfig {
    let (consensus_public_key, consensus_secret_key) = exonum::crypto::gen_keypair();
    let (service_public_key, service_secret_key) = exonum::crypto::gen_keypair();
    let validator_keys = ValidatorKeys {
        consensus_key: consensus_public_key,
        service_key: service_public_key,
    };
    let genesis = GenesisConfig::new(
        vec![
            validator_keys,
        ].into_iter()
    );

}

fn main() {
    dotenv::dotenv().ok();
    exonum::helpers::init_logger().unwrap();
    info!("RustFest Cryptoexchange");
    let node = Node::new(
        MemoryDB::new(),
        vec![],
        node_config(),
    );
}
