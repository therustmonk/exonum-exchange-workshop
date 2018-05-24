use stdweb::Value;
use stdweb::unstable::TryInto;

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

    pub fn get_owner(&mut self) -> String {
        let value = js! {
            return keyPair.publicKey;
        };
        value.try_into().unwrap()
    }

    pub fn order_transaction(&mut self) {
    }
}
