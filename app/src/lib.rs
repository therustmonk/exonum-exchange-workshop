#![recursion_limit="256"]
extern crate failure;
#[macro_use]
extern crate log;
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
extern crate rand;

pub mod context;
pub mod exonum;

use yew::prelude::*;
use yew::services::interval::IntervalTask;
use yew::services::fetch::FetchTask;
use context::{Context, Account, OrderBook, Order};

pub struct Model {
    interval_task: IntervalTask,
    account_task: Option<FetchTask>,
    orders_task: Option<FetchTask>,
    account: Option<Account>,
    order_book: Option<OrderBook>,
}

pub enum Msg {
    Account(Result<Account, String>),
    OrderBook(Result<OrderBook, String>),
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    NeedUpdate(()),
    PutOrder,
    Cancel(u32),
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
            account_task: None,
            orders_task: None,
            account: None,
            order_book: None,
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
            Msg::OrderBook(order_book) => {
                info!("Order Book: {:?}", order_book);
                if let Ok(order_book) = order_book {
                    self.order_book = Some(order_book);
                }
            },
            Msg::NeedUpdate(_) => {
                let callback = env.send_back(Msg::Account);
                let task = env.fetch_account(callback);
                self.account_task = Some(task);
                let callback = env.send_back(Msg::OrderBook);
                let task = env.fetch_orders(callback);
                self.orders_task = Some(task);
            },
            Msg::PutOrder => {
                env.exonum().put_order();
            },
            Msg::Cancel(id) => {
                env.exonum().cancel_order(id);
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
                            { self.view_order_book() }
                        </div>
                        <div class="column",>
                            { self.view_account() }
                        </div>
                    </div>
                </div>
            </section>
        }
    }
}

impl Model {
    fn view_account(&self) -> Html<Context, Self> {
        if let Some(ref account) = self.account {
            let view_order_id = |id: &u32| {
                let order_id = *id;
                html! {
                    <li onclick=|_| Msg::Cancel(order_id),>{ id }</li>
                }
            };
            html! {
                <div>
                    <div class="section",>
                        <div class="box",>
                            <div>{ format!("OWNER: {}", account.owner) }</div>
                            <div>{ format!("USD: {}", account.usd_balance) }</div>
                            <div>{ format!("TOKEN: {}", account.token_balance) }</div>
                        </div>
                    </div>
                    <div class="section",>
                        <div class="box",>
                            <div class="field",>
                                <label class="label",>{ "Price" }</label>
                                <div class="control",>
                                <input class="input", type="text", placeholder="Price", />
                                </div>
                            </div>
                            <div class="field",>
                                <label class="label",>{ "Amount" }</label>
                                <div class="control",>
                                <input class="input", type="text", placeholder="Amount", />
                                </div>
                            </div>
                            <div class=("field", "has-addons"),>
                                <p class="control",>
                                    <button class="button", onclick=|_| Msg::PutOrder,>{ "Buy" }</button>
                                </p>
                                <p class="control",>
                                    <button class="button", onclick=|_| Msg::PutOrder,>{ "Sell" }</button>
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="section",>
                        <p class="title",>{ "Orders" }</p>
                        <ul>
                            { for account.orders.iter().map(view_order_id) }
                        </ul>
                    </div>
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


    fn view_order_book(&self) -> Html<Context, Self> {
        if let Some(ref order_book) = self.order_book {
            let view_side = |buy: bool| {
                if buy {
                    html! {
                        <span class=("button", "is-small", "is-success"),>{ "Buy" }</span>
                    }
                } else {
                    html! {
                        <span class=("button", "is-small", "is-danger"),>{ "Sell" }</span>
                    }
                }
            };
            let view_order = |(_, order): (&u32, &Order)| html! {
                <tr>
                    <td>{ order.id }</td>
                    <td>{ order.price }</td>
                    <td>{ order.amount.abs() }</td>
                    <td>{ view_side(order.amount > 0) }</td>
                </tr>
            };
            html! {
                <table class=("table", "is-fullwidth"),>
                    <thead>
                        <tr>
                            <th>{ "#ID" }</th>
                            <th>{ "Price" }</th>
                            <th>{ "Amount" }</th>
                            <th>{ "Side" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for order_book.iter().map(view_order) }
                    </tbody>
                </table>
            }
        } else {
            html! {
                <div>
                    <p>{ "Not loaded" }</p>
                </div>
            }
        }
    }
}
