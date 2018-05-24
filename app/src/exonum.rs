use stdweb::Value;

#[derive(Debug)]
pub struct KeyPair(Value);

pub struct ExonumService {
}

impl ExonumService {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn keypair(&mut self) -> KeyPair {
        let value = js! {
            createAccount();
        };
        KeyPair(value)
    }

    pub fn order_transaction(&mut self) {
    }
}
