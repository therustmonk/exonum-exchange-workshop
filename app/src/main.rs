extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;
extern crate stdweb;
extern crate hex;
extern crate byteorder;

use stdweb::web::document;
use stdweb::web::{INode, IParentNode};
use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use byteorder::{LittleEndian, WriteBytesExt};

trait Fill {
    fn fill(&self) -> Vec<u8>;
}

struct Message<T> {
    network_id: u8,
    protocol_version: u8,
    service_id: u16,
    message_type: u16,
    data: T,
}

const SERVICE_DATA_LEN: usize = 10; // bytes
const SIGNATURE_LEN: usize = 64; // bytes

impl<T: Fill> Fill for Message<T> {
    fn fill(&self) -> Vec<u8> {
        let payload = self.data.fill();
        let mut buffer = Vec::new();
        buffer.write_u8(self.network_id);
        buffer.write_u8(self.protocol_version);
        buffer.write_u16::<LittleEndian>(self.message_type);
        buffer.write_u16::<LittleEndian>(self.service_id);
        let len = SERVICE_DATA_LEN + payload.len() + SIGNATURE_LEN;
        buffer.write_u32::<LittleEndian>(len as u32);
        buffer.extend_from_slice(&payload);
        buffer
    }
}

impl<T: Fill> Message<T> {
    fn to_exonum(&self, keypair: &Keypair) {
        let data = self.fill();
        let signature: Signature = keypair.sign::<Sha512>(&data);
    }
}

struct TxCreate {
    owner: PublicKey,
}

impl Fill for TxCreate {
    fn fill(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(self.owner.as_bytes());
        buffer
    }
}

fn main() {
    stdweb::initialize();
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    println!("Keypair: {:?}", keypair);
    let body = document().query_selector( "body" ).unwrap().unwrap();
    let div = document().create_element( "div" ).unwrap();
    let pkey = hex::encode(keypair.public.to_bytes());
    let message = format!("Public Key: {}", pkey);
    div.set_text_content(&message);
    body.append_child(&div);
    stdweb::event_loop();
}
