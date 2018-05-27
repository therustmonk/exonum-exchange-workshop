#[macro_use]
extern crate log;
extern crate dotenv;
extern crate exonum;

fn main() {
    dotenv::dotenv().ok();
    exonum::helpers::init_logger().unwrap();
    info!("RustFest Cryptoexchange");
    println!("Hello, world!");
}
