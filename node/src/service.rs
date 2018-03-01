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
use bodyparser;
use serde_json;
use protocol::*;

// // // // // // // // // // CONSTANTS // // // // // // // // // //

const USD_BALANCE: u64 = 1000;
const TOKEN_BALANCE: u64 = 1000000;

// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //

encoding_struct! {
    struct Account {
        owner: &PublicKey,
        usd_balance: u64,
        token_balance: u64,
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
            let account = Account::new(self.owner(), USD_BALANCE, TOKEN_BALANCE);
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

impl ExchangeServiceApi {
    fn post_transaction<T>(&self, req: &mut Request) -> IronResult<Response>
    where
        T: Transaction + Clone + for<'de> Deserialize<'de>,
    {
        match req.get::<bodyparser::Struct<T>>() {
            Ok(Some(transaction)) => {
                let transaction: Box<Transaction> = Box::new(transaction);
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                self.ok_response(&json!({
                    "tx_hash": tx_hash
                }))
            }
            Ok(None) => Err(ApiError::IncorrectRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::IncorrectRequest(Box::new(e)))?,
        }
}
}

impl Api for ExchangeServiceApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_create_account =
            move |req: &mut Request| self_.post_transaction::<TxCreate>(req);
        router.post("/v1/account", post_create_account, "post_create_account");
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
