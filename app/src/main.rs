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

use std::collections::HashMap;
use stdweb::web::document;
use stdweb::web::{XmlHttpRequest, INode, IParentNode};
use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use byteorder::{LittleEndian, WriteBytesExt};
use serde::Serialize;

trait Fill {
    fn fill(&self) -> Vec<u8>;
    fn to_value(&self) -> serde_json::Value;
}

struct Message<T> {
    network_id: u8,
    protocol_version: u8,
    service_id: u16,
    message_id: u16,
    body: T,
}

const SERVICE_DATA_LEN: usize = 10; // bytes
const SIGNATURE_LEN: usize = 64; // bytes

impl<T: Fill> Fill for Message<T> {
    fn fill(&self) -> Vec<u8> {
        let payload = self.body.fill();
        let mut buffer = Vec::new();
        buffer.write_u8(self.network_id);
        buffer.write_u8(self.protocol_version);
        buffer.write_u16::<LittleEndian>(self.message_id);
        buffer.write_u16::<LittleEndian>(self.service_id);
        let len = SERVICE_DATA_LEN + payload.len() + SIGNATURE_LEN;
        buffer.write_u32::<LittleEndian>(len as u32);
        buffer.extend_from_slice(&payload);
        buffer
    }

    fn to_value(&self) -> serde_json::Value {
        let mut map: HashMap<String, serde_json::Value> = HashMap::new();
        map.insert("network_id".into(), self.network_id.into());
        map.insert("protocol_version".into(), self.protocol_version.into());
        map.insert("message_id".into(), self.message_id.into());
        map.insert("service_id".into(), self.service_id.into());
        map.insert("body".into(), self.body.to_value());
        serde_json::to_value(&map).unwrap()
    }
}

impl<T: Fill> Message<T> {
    fn to_exonum(&self, keypair: &Keypair) -> String {
        let data = self.fill(); // DRY
        let signature: Signature = keypair.sign::<Sha512>(&data);
        let mut value = self.to_value();
        {
            let size = self.body.fill().len();
            let object = value.as_object_mut().unwrap();
            object.insert("size".into(), size.into());
            let signature = hex::encode(signature.to_bytes().as_ref());
            object.insert("signature".into(), signature.into());
        }
        serde_json::to_string_pretty(&value).unwrap()
    }
}

struct TxCreate {
    owner: [u8; 32],
}

impl Fill for TxCreate {
    fn fill(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.owner);
        buffer
    }

    fn to_value(&self) -> serde_json::Value {
        let mut map: HashMap<String, serde_json::Value> = HashMap::new();
        map.insert("owner".into(), hex::encode(&self.owner).into());
        serde_json::to_value(&map).unwrap()
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
