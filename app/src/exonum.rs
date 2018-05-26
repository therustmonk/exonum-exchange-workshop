use stdweb::Value;
use stdweb::unstable::TryInto;
use rand::prelude::*;

#[derive(Debug)]
pub struct KeyPair(Value);

pub struct ExonumService {
}

impl ExonumService {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn create_account(&mut self) {
        js! {
            createAccount();
        };
    }

    pub fn put_order(&mut self) {
        let id: u32 = random();
        js! {
            let id = @{id};
            putOrder(1000, 10, id);
        };
    }

    pub fn cancel_order(&mut self, id: u32) {
        js! {
            let id = @{id};
            cancelOrder(id);
        };
    }

    pub fn get_owner(&mut self) -> String {
        let value = js! {
            return keyPair.publicKey;
        };
        value.try_into().unwrap()
    }

    pub fn order_transaction(&mut self) {
    }
}
