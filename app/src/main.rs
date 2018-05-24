extern crate web_logger;
extern crate yew;
extern crate trading;

use yew::prelude::*;
use yew::services::fetch::FetchService;
use trading::context::Context;
use trading::Model;

fn main() {
    web_logger::init();
    yew::initialize();
    let context = Context::new();
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}

/*
use std::rc::Rc;
use stdweb::web::document;
use stdweb::web::{XmlHttpRequest, IEventTarget, INode, IParentNode};
use stdweb::web::event::ClickEvent;
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;
use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use client::{Message, TxCreate, TxOrder};
use serde_json::Value;

const UPDATE_MS: u32 = 1_000;

fn main() {
    stdweb::initialize();
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    let keypair = Rc::new(keypair);
    println!("Keypair: {:?}", keypair);
    let pkey = hex::encode(keypair.public.to_bytes());
    update_text("#key", &pkey);

    let buy = document().query_selector("#buy").unwrap().unwrap();
    let keypair_ = keypair.clone();
    buy.add_event_listener(move |_: ClickEvent| {
        place_order(false, keypair_.clone());
    });

    let sell = document().query_selector("#sell").unwrap().unwrap();
    let keypair_ = keypair.clone();
    sell.add_event_listener(move |_: ClickEvent| {
        place_order(true, keypair_.clone());
    });

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
    let data = message.to_exonum(keypair.clone());
    fetch("POST", "http://localhost:8080/api/services/cryptoexchange/v1/account", Some(&data), |success, data| {
        println!("Data: {}", data);
    });

    stdweb::web::set_timeout(move || update_account(keypair), UPDATE_MS);

    stdweb::event_loop();
}

#[derive(Deserialize)]
struct Account {
    owner: String,
    token_balance: String,
    usd_balance: String,
}

fn update_account(keypair: Rc<Keypair>) {
    let request = XmlHttpRequest::new();
    let pub_key = hex::encode(keypair.public.as_bytes());
    let url = format!("http://localhost:8080/api/services/cryptoexchange/v1/account/{}", pub_key);
    fetch("GET", &url, None, |success, data| {
        if let Ok(acc) = serde_json::from_str::<Account>(&data) {
            update_text("#usd", &acc.usd_balance);
            update_text("#tok", &acc.token_balance);
        }
    });
    stdweb::web::set_timeout(move || update_account(keypair), UPDATE_MS);
}

fn fetch<F>(method: &str, url: &str, body: Option<&str>, callback: F)
where
    F: Fn(bool, String) + 'static,
{
    let value = js! {
        var data = {
            method: @{method},
            body: @{body},
        };
        var request = new Request(@{url}, data);
        var callback = @{callback};
        fetch(request).then(function(response) {
            if (response.ok) {
                return response.text();
            } else {
                throw new Error("Network response was not ok.");
            }
        }).then(function(data) {
            console.log(data);
            callback(true, data);
            callback.drop();
        }).catch(function(err) {
            console.log(err);
            callback(false, err.message);
            callback.drop();
        });
    };
}

fn update_text(query: &str, value: &str) {
    let elem = document().query_selector(query).unwrap().unwrap();
    elem.set_text_content(value);
}

fn append_order(tx_order: &TxOrder) {
    let orders = document().query_selector("#orders").unwrap().unwrap();
    let order = document().create_element("div").unwrap();
    orders.append_child(&order);

    let uid = document().create_element("p").unwrap();
    let value = format!("Id: {}", tx_order.id);
    uid.set_text_content(&value);
    order.append_child(&uid);

    let price = document().create_element("p").unwrap();
    let value = format!("Price: {}", tx_order.price);
    price.set_text_content(&value);
    order.append_child(&price);

    let amount = document().create_element("p").unwrap();
    let value = format!("Size: {}", tx_order.amount);
    amount.set_text_content(&value);
    order.append_child(&amount);
}

fn place_order(sell: bool, keypair: Rc<Keypair>) {
    let price = document().query_selector("#price").unwrap().unwrap();
    let price: InputElement = price.try_into().unwrap();
    let price = price.raw_value().parse::<u32>();
    let size = document().query_selector("#size").unwrap().unwrap();
    let size: InputElement = size.try_into().unwrap();
    let size = size.raw_value().parse::<u32>();
    println!("{:?}, {:?}", price, size);
    let uid = rand::random::<u32>();
    if let (Ok(price), Ok(size)) = (price, size) {
        let size = if sell { -(size as i32) } else { size as i32 };
        let tx_order = TxOrder {
            owner: keypair.public.to_bytes(),
            price: price,
            amount: size,
            id: uid,
        };
        append_order(&tx_order);
        let message = Message {
            network_id: 0,
            protocol_version: 0,
            service_id: 1,
            message_id: 2,
            body: tx_order,
        };
        let data = message.to_exonum(keypair);
        fetch("POST", "http://localhost:8080/api/services/cryptoexchange/v1/order", Some(&data), |success, data| {
            println!("Order: {}", data);
        });
    }
}
*/
