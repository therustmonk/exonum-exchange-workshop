use std::collections::HashMap;
use std::rc::Rc;
use serde_json::{self, Value};
use hex;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use byteorder::{LittleEndian, WriteBytesExt};

pub trait Fill {
    fn fill(&self) -> Vec<u8>;
    fn to_value(&self) -> Value;
}

pub struct Message<T> {
    pub network_id: u8,
    pub protocol_version: u8,
    pub service_id: u16,
    pub message_id: u16,
    pub body: T,
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

    fn to_value(&self) -> Value {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("network_id".into(), self.network_id.into());
        map.insert("protocol_version".into(), self.protocol_version.into());
        map.insert("message_id".into(), self.message_id.into());
        map.insert("service_id".into(), self.service_id.into());
        map.insert("body".into(), self.body.to_value());
        serde_json::to_value(&map).unwrap()
    }
}

impl<T: Fill> Message<T> {
    pub fn to_exonum(&self, keypair: Rc<Keypair>) -> String {
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

pub struct TxCreate {
    pub owner: [u8; 32],
}

impl Fill for TxCreate {
    fn fill(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.owner);
        buffer
    }

    fn to_value(&self) -> Value {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("owner".into(), hex::encode(&self.owner).into());
        serde_json::to_value(&map).unwrap()
    }
}

pub struct TxOrder {
    pub owner: [u8; 32],
    pub price: u32,
    pub amount: i32,
    pub id: u32,
}

impl Fill for TxOrder {
    fn fill(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&self.owner);
        buffer.write_u32::<LittleEndian>(self.price);
        buffer.write_i32::<LittleEndian>(self.amount);
        buffer.write_u32::<LittleEndian>(self.id);
        buffer
    }

    fn to_value(&self) -> Value {
        let mut map: HashMap<String, Value> = HashMap::new();
        map.insert("owner".into(), hex::encode(&self.owner).into());
        map.insert("price".into(), self.price.into());
        map.insert("amount".into(), self.amount.into());
        map.insert("id".into(), self.id.into());
        serde_json::to_value(&map).unwrap()
    }
}
