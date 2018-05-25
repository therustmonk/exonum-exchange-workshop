#![recursion_limit="256"]
extern crate failure;
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
use yew::services::interval::IntervalTask;
use yew::services::fetch::FetchTask;
use context::{Context, Account, OrderBook, Order};

pub struct Model {
    interval_task: IntervalTask,
    task: Option<FetchTask>,
    account: Option<Account>,
}

pub enum Msg {
    Account(Result<Account, String>),
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    NeedUpdate(()),
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<Context, Self>) -> Self {
        env.exonum().create_account();
        let callback = env.send_back(Msg::NeedUpdate);
        let interval_task = env.schedule_updates(callback);
        Model {
            interval_task,
            task: None,
            account: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Account(account) => {
                info!("Account: {:?}", account);
                if let Ok(account) = account {
                    self.account = Some(account);
                }
            },
            Msg::NeedUpdate(_) => {
                let callback = env.send_back(Msg::Account);
                let task = env.fetch_account(callback);
                self.task = Some(task);
            },
            _ => {
            },
        }
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
                        <div class="column",>
                            { self.view_account() }
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

impl Model {
    fn view_account(&self) -> Html<Context, Self> {
        if let Some(ref account) = self.account {
            let view_order = |order: &Order| html! {
                <li>{ order.id }</id>
            };
            html! {
                <div>
                    <div>{ format!("OWNER: {}", account.owner) }</div>
                    <div>{ format!("USD: {}", account.usd_balance) }</div>
                    <div>{ format!("TOKEN: {}", account.token_balance) }</div>
                    <ul>
                        { for account.orders.iter().map(view_order) }
                    </ul>
                </div>
            }
        } else {
            html! {
                <div>
                    <div>{ "Not loaded" }</div>
                </div>
            }
        }
    }
}
