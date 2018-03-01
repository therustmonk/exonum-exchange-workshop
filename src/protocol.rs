use exonum::crypto::PublicKey;

pub const SERVICE_ID: u16 = 1;

pub const TX_CREATE: u16 = 1;
//pub const TX_CALL: u16 = 2;

message! {
    struct TxCreate {
        const TYPE = SERVICE_ID;
        const ID = TX_CREATE;

        owner: &PublicKey,
    }
}
