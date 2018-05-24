#![recursion_limit="256"]
#[macro_use]
extern crate log;
extern crate rand;
extern crate sha2;
extern crate ed25519_dalek;
extern crate hex;
extern crate byteorder;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

pub mod context;
pub mod exonum;

use yew::prelude::*;
use context::Context;
use exonum::{ExonumService, KeyPair};

pub struct Model {
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<Context, Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Context, Self>) -> ShouldRender {
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <section class=("section", "is-fullheight"),>
                <div class="container",>
                    <div class="columns",>
                        <div class="column",>
                            <nav class="level",>
                                <p class=("level-item", "title", "has-text-centered"),>
                                    { "Orders" }
                                </p>
                            </nav>
                            <table class="table",>
                            </table>
                        </div>
                    </div>
                    <button class="button", onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </div>
            </section>
        }
    }
}
