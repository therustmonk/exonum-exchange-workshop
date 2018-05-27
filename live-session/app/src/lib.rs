#![recursion_limit="512"]
#[macro_use]
extern crate log;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub type Context = ();

pub struct Model {
    price_text: String,
    orders: Vec<String>,
}

pub enum Msg {
    UpdatePriceText(String),
    SendOrder,
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, env: &mut Env<Context, Self>) -> Self {
        Model {
            price_text: "".to_string(),
            orders: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::UpdatePriceText(text) => {
                debug!("You've entered: {}", text);
                self.price_text = text;
                true
            },
            Msg::SendOrder => {
                warn!("Order sent");
                let mut order = format!("");
                ::std::mem::swap(&mut self.price_text, &mut order);
                self.orders.push(order);
                true
            },
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        let price_class = {
            if self.price_text.trim().is_empty() { "is-danger" } else { "is-normal" }
        };
        html! {
            <div class="container",>
                <div class="section",>
                    <div class=("form", "box"),>
                        <div class="field",>
                            <label class="label",>{ "Price" }</label>
                            <div class="control",>
                                <input class=("input", price_class),
                                       type="text",
                                       value=&self.price_text,
                                       placeholder="What price is it?",
                                       oninput=|event| Msg::UpdatePriceText(event.value),
                                       />
                            </div>
                        </div>
                        <div class="field",>
                            <div class="control",>
                                <button class=("button", "is-primary"),
                                        onclick=|_| Msg::SendOrder,
                                        >{ "Send Order" }</button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="section",>
                    <table class=("table", "is-fullwidth"),>
                        <thead>
                            <tr>
                                <th>{ "Price" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            { for self.orders.iter().map(view_order) }
                        </tbody>
                    </table>
                </div>
            </div>
        }
    }
}

fn view_order(order: &String) -> Html<Context, Model> {
    html! {
        <tr>
            <td>{ order }</td>
        </tr>
    }
}
