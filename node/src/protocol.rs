use exonum::crypto::PublicKey;

pub const SERVICE_ID: u16 = 1;

pub const TX_CREATE: u16 = 1;
pub const TX_ORDER: u16 = 2;

transactions! {
    pub Transactions {
        const SERVICE_ID = SERVICE_ID;

        struct TxCreate {
            owner: &PublicKey,
        }
        struct TxOrder {
            owner: &PublicKey,
            price: u32,
            amount: i32,
            id: u32,
        }
    }
}
