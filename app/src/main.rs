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
    let div = document().query_selector("#key").unwrap().unwrap();
    let pkey = hex::encode(keypair.public.to_bytes());
    div.set_text_content(&pkey);

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
    fetch("POST", "http://localhost:8080/api/services/cryptoexchange/v1/account", &data, |success, data| {
        println!("Data: {}", data);
    });
    /*
    request.open("POST", "http://localhost:8080/api/services/cryptoexchange/v1/account");
    request.send_with_string(&data);
    let cloned = request.clone();
    request.add_event_listener(move |_: ResourceLoadEvent| {
        cloned.response_text().unwrap().unwrap();
    });
    */
    //stdweb::web::set_timeout(move || update_account(keypair), 10_000);

    stdweb::event_loop();
}

fn update_account(keypair: Keypair) {
    let request = XmlHttpRequest::new();
    let pub_key = hex::encode(keypair.public.as_bytes());
    let url = format!("http://localhost:8080/api/services/cryptoexchange/v1/account/{}", pub_key);
    request.open("GET", &url);
    request.send();
    stdweb::web::set_timeout(move || update_account(keypair), 10_000);
}

fn fetch<F>(method: &str, url: &str, body: &str, callback: F)
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
