#![recursion_limit="256"]
extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;
#[macro_use]
extern crate stdweb;
extern crate hex;
extern crate byteorder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod client;

use stdweb::web::document;
use stdweb::web::{XmlHttpRequest, IEventTarget, INode, IParentNode};
use stdweb::web::event::ResourceLoadEvent;
use rand::Rng;
use rand::OsRng;
use sha2::Sha512;
use ed25519_dalek::{Keypair, Signature, PublicKey};
use client::{Message, TxCreate};
use serde_json::Value;

fn main() {
    stdweb::initialize();
    let mut cspring: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate::<Sha512>(&mut cspring);
    println!("Keypair: {:?}", keypair);
    let pkey = hex::encode(keypair.public.to_bytes());
    update_text("#key", &pkey);

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
    let data = message.to_exonum(&keypair);
    fetch("POST", "http://localhost:8080/api/services/cryptoexchange/v1/account", Some(&data), |success, data| {
        println!("Data: {}", data);
    });

    stdweb::web::set_timeout(move || update_account(keypair), 3_000);

    stdweb::event_loop();
}

#[derive(Deserialize)]
struct Account {
    owner: String,
    token_balance: String,
    usd_balance: String,
}

fn update_account(keypair: Keypair) {
    let request = XmlHttpRequest::new();
    let pub_key = hex::encode(keypair.public.as_bytes());
    let url = format!("http://localhost:8080/api/services/cryptoexchange/v1/account/{}", pub_key);
    fetch("GET", &url, None, |success, data| {
        if let Ok(acc) = serde_json::from_str::<Account>(&data) {
            update_text("#usd", &acc.usd_balance);
            update_text("#tok", &acc.token_balance);
        }
    });
    stdweb::web::set_timeout(move || update_account(keypair), 3_000);
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
