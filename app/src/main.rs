extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;
extern crate stdweb;

use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

fn main() {
    stdweb::initialize();
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    println!("Keypair: {:?}", keypair);
    stdweb::event_loop();
}
