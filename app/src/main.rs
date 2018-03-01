extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;
extern crate stdweb;
extern crate hex;
extern crate byteorder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod client;

use stdweb::web::document;
use stdweb::web::{XmlHttpRequest, INode, IParentNode};
use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use client::{Message, TxCreate};

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

    let request = XmlHttpRequest::new();
    let tx_create = TxCreate {
        owner: keypair.public.to_bytes(),
    };
    let message = Message {
        network_id: 0,
        protocol_version: 0,
        service_id: 1,
        message_id: 1,
        body: tx_create,
    };
    request.open("POST", "http://localhost:8080/api/services/cryptoexchange/v1/account");
    let data = message.to_exonum(&keypair);
    request.send_with_string(&data);

    stdweb::event_loop();
}
