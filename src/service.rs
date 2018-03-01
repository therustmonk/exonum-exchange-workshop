use std::io::Read;
use exonum::blockchain::{Blockchain, Service, Transaction, ApiContext};
use exonum::node::{TransactionSend, ApiSender};
use exonum::messages::{RawTransaction, Message};
use exonum::storage::{Fork, MapIndex, Snapshot};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::Handler;
use router::Router;
use serde::Deserialize;
use serde_json;
use protocol::*;

// // // // // // // // // // CONSTANTS // // // // // // // // // //

const INIT_BALANCE: u64 = 1000000;

// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //

encoding_struct! {
    struct Account {
        owner: &PublicKey,
        balance: u64,
    }
}

// // // // // // // // // // DATA LAYOUT // // // // // // // // // //

pub struct ExchangeSchema<T> {
    view: T,
}

impl<T: AsRef<Snapshot>> ExchangeSchema<T> {
    pub fn new(view: T) -> Self {
        ExchangeSchema { view }
    }

    pub fn accounts(&self) -> MapIndex<&Snapshot, PublicKey, Account> {
        MapIndex::new("cryptoexchange.accounts", self.view.as_ref())
    }

    pub fn account(&self, owner: &PublicKey) -> Option<Account> {
        self.accounts().get(owner)
    }
}

impl<'a> ExchangeSchema<&'a mut Fork> {
    pub fn accounts_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Account> {
        MapIndex::new("cryptoexchange.accounts", &mut self.view)
    }
}

// // // // // // // // // // CONTRACTS // // // // // // // // // //

impl Transaction for TxCreate {
    fn verify(&self) -> bool {
        self.verify_signature(self.owner())
    }

    fn execute(&self, view: &mut Fork) {
        let mut schema = ExchangeSchema::new(view);
        if schema.account(self.owner()).is_none() {
            let account = Account::new(self.owner(), INIT_BALANCE);
            println!("Create the account: {:?}", account);
            schema.accounts_mut().put(self.owner(), account);
        }
    }
}

// // // // // // // // // // REST API // // // // // // // // // //

#[derive(Clone)]
struct ExchangeServiceApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl Api for ExchangeServiceApi {
    fn wire(&self, router: &mut Router) {
    }
}

// // // // // // // // // // SERVICE DECLARATION // // // // // // // // // //
pub struct ExchangeService;

impl Service for ExchangeService {
    fn service_name(&self) -> &'static str {
        "cryptoexchange"
    }

    fn service_id(&self) -> u16 {
        SERVICE_ID
    }

    fn tx_from_raw(&self, raw: RawTransaction) -> Result<Box<Transaction>, encoding::Error> {
        let trans: Box<Transaction> = match raw.message_type() {
            TX_CREATE => Box::new(TxCreate::from_raw(raw)?),
            _ => {
                return Err(encoding::Error::IncorrectMessageType {
                    message_type: raw.message_type(),
                });
            }
        };
        Ok(trans)
    }

    fn state_hash(&self, _: &Snapshot) -> Vec<Hash> {
        vec![]
    }

    fn public_api_handler(&self, ctx: &ApiContext) -> Option<Box<Handler>> {
        let mut router = Router::new();
        let api = ExchangeServiceApi {
            channel: ctx.node_channel().clone(),
            blockchain: ctx.blockchain().clone(),
        };
        api.wire(&mut router);
        Some(Box::new(router))
    }
}
