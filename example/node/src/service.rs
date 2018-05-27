use std::collections::BTreeMap;
use exonum::blockchain::{
    Blockchain,
    Service,
    Transaction,
    TransactionSet,
    ApiContext,
    ExecutionResult,
};
use exonum::node::{TransactionSend, ApiSender};
use exonum::messages::{RawTransaction, Message};
use exonum::storage::{Fork, MapIndex, Snapshot};
use exonum::crypto::{Hash, PublicKey};
use exonum::encoding;
use exonum::encoding::serialize::FromHex;
use exonum::api::{Api, ApiError};
use iron::prelude::*;
use iron::Handler;
use router::Router;
use bodyparser;
use serde_json;
use protocol::*;

// // // // // // // // // // CONSTANTS // // // // // // // // // //

const USD_BALANCE: u32 = 1_000_000;
const TOKEN_BALANCE: u32 = 1_000;

// // // // // // // // // // PERSISTENT DATA // // // // // // // // // //

encoding_struct! {
    struct Account {
        owner: &PublicKey,
        usd_balance: u32, // .00
        token_balance: u32, // .000000
        orders: Vec<u32>,
    }
}

impl Account {
    fn buy_tokens(&self, price: u32, amount: i32, id: u32) -> Self {
        let usd_balance = self.usd_balance() - (price as i32 * amount) as u32;
        let token_balance = self.token_balance() + amount as u32;
        let mut orders = self.orders();
        orders.push(id);
        Self::new(self.owner(), usd_balance, token_balance, orders)
    }

    fn sell_tokens(&self, price: u32, amount: i32, id: u32) -> Self {
        let usd_balance = self.usd_balance() + (price as i32 * amount) as u32;
        let token_balance = self.token_balance() - amount as u32;
        let mut orders = self.orders();
        orders.push(id);
        Self::new(self.owner(), usd_balance, token_balance, orders)
    }

    fn add_order_id(&self, id: u32) -> Self {
        let mut orders = self.orders();
        orders.push(id);
        Self::new(self.owner(), self.usd_balance(), self.token_balance(), orders)
    }

    fn remove_order_by_id(&self, id: u32) -> Option<Self> {
        let mut orders = self.orders();
        if let Some(index) = orders.iter().position(|x| *x == id) {
            orders.remove(index);
            let res = Self::new(self.owner(), self.usd_balance(), self.token_balance(), orders);
            Some(res)
        } else {
            None
        }
    }
}

encoding_struct! {
    struct Order {
        owner: &PublicKey,
        price: u32,
        amount: i32,
        id: u32,
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

    pub fn orders(&self) -> MapIndex<&Snapshot, u32, Order> {
        MapIndex::new("cryptoexchange.orders", self.view.as_ref())
    }
}

impl<'a> ExchangeSchema<&'a mut Fork> {
    pub fn accounts_mut(&mut self) -> MapIndex<&mut Fork, PublicKey, Account> {
        MapIndex::new("cryptoexchange.accounts", &mut self.view)
    }

    pub fn orders_mut(&mut self) -> MapIndex<&mut Fork, u32, Order> {
        MapIndex::new("cryptoexchange.orders", &mut self.view)
    }
}

// // // // // // // // // // CONTRACTS // // // // // // // // // //

impl Transaction for TxCreate {
    fn verify(&self) -> bool {
        self.verify_signature(self.owner())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        trace!("TxOrder");
        let mut schema = ExchangeSchema::new(view);
        if schema.account(self.owner()).is_none() {
            let account = Account::new(self.owner(), USD_BALANCE, TOKEN_BALANCE, Vec::new());
            println!("Create the account: {:?}", account);
            schema.accounts_mut().put(self.owner(), account);
        }
        Ok(())
    }
}

impl Transaction for TxOrder {
    fn verify(&self) -> bool {
        self.verify_signature(self.owner())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        trace!("TxOrder");
        let mut schema = ExchangeSchema::new(view);
        let account = schema.account(self.owner());
        if let Some(account) = account {
            let not_exists = !schema.orders_mut().contains(&self.id());
            if not_exists {
                let order = Order::new(self.owner(), self.price(), self.amount(), self.id());
                println!("Put the order <{}>: {:?}", self.id(), order);
                let account = account.add_order_id(self.id());
                schema.orders_mut().put(&self.id(), order);
                schema.accounts_mut().put(self.owner(), account);
                /*
                let account = {
                    if order.amount() > 0 {
                        account.buy_tokens(order.price(), order.amount(), order.id())
                    } else {
                        account.sell_tokens(order.price(), -order.amount(), order.id())
                    }
                };
                */
            }
        }
        Ok(())
    }
}

impl Transaction for TxCancel {
    fn verify(&self) -> bool {
        self.verify_signature(self.owner())
    }

    fn execute(&self, view: &mut Fork) -> ExecutionResult {
        trace!("TxCancel");
        let mut schema = ExchangeSchema::new(view);
        let account = schema.account(self.owner());
        if let Some(account) = account {
            let exists = schema.orders_mut().contains(&self.id());
            if exists {
                if let Some(account) = account.remove_order_by_id(self.id()) {
                    schema.orders_mut().remove(&self.id());
                    schema.accounts_mut().put(self.owner(), account);
                }
            }
        }
        Ok(())
    }
}

// // // // // // // // // // REST API // // // // // // // // // //

#[derive(Clone)]
struct ExchangeServiceApi {
    channel: ApiSender,
    blockchain: Blockchain,
}

impl ExchangeServiceApi {
    fn post_transaction(&self, req: &mut Request) -> IronResult<Response> {
        info!("Transaction...");
        match req.get::<bodyparser::Struct<Transactions>>() {
            Ok(Some(transaction)) => {
                info!("Ok...");
                let transaction: Box<Transaction> = transaction.into();
                let tx_hash = transaction.hash();
                self.channel.send(transaction).map_err(ApiError::from)?;
                self.ok_response(&json!({
                    "tx_hash": tx_hash
                }))
            }
            Ok(None) => Err(ApiError::BadRequest("Empty request body".into()))?,
            Err(e) => Err(ApiError::BadRequest(e.to_string()))?,
        }
    }

    fn get_account(&self, req: &mut Request) -> IronResult<Response> {
        let path = req.url.path();
        let wallet_key = path.last().unwrap();
        let public_key = PublicKey::from_hex(wallet_key)
            .map_err(|e| ApiError::BadRequest(e.to_string()))?;

        let account = {
            let snapshot = self.blockchain.snapshot();
            let schema = ExchangeSchema::new(snapshot);
            schema.account(&public_key)
        };

        if let Some(account) = account {
            self.ok_response(&serde_json::to_value(account).unwrap())
        } else {
            self.not_found_response(&serde_json::to_value("account not found").unwrap())
        }
    }

    fn get_orders(&self, req: &mut Request) -> IronResult<Response> {
        let snapshot = self.blockchain.snapshot();
        let schema = ExchangeSchema::new(snapshot);
        let orders = schema.orders();
        let orders = orders.iter().collect::<BTreeMap<u32, Order>>();

        self.ok_response(&serde_json::to_value(orders).unwrap())
    }
}

impl Api for ExchangeServiceApi {
    fn wire(&self, router: &mut Router) {
        let self_ = self.clone();
        let post_create_account =
            move |req: &mut Request| self_.post_transaction(req);
        let self_ = self.clone();
        let post_create_order =
            move |req: &mut Request| self_.post_transaction(req);
        let self_ = self.clone();
        let post_cancel_order =
            move |req: &mut Request| self_.post_transaction(req);
        let self_ = self.clone();
        let get_account = move |req: &mut Request| self_.get_account(req);
        let self_ = self.clone();
        let get_orders = move |req: &mut Request| self_.get_orders(req);
        router.post("/v1/account", post_create_account, "post_create_account");
        router.post("/v1/order", post_create_order, "post_create_order");
        router.post("/v1/cancel", post_cancel_order, "post_cancel_order");
        router.get("/v1/account/:pub_key", get_account, "get_account");
        router.get("/v1/orders", get_orders, "get_orders");
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
        let tx = Transactions::tx_from_raw(raw)?;
        Ok(tx.into())
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
