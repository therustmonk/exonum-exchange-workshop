#[macro_use]
extern crate log;
#[macro_use]
extern crate yew;

use yew::prelude::*;

pub type Context = ();

pub struct Model {
    price_text: String,
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
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::UpdatePriceText(text) => {
                debug!("You've entered: {}", text);
                self.price_text = text;
                false
            },
            Msg::SendOrder => {
                warn!("Order sent");
                true
            },
        }
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div class="container",>
                <div class="section",>
                    <div class=("form", "box"),>
                        <div class="field",>
                            <label class="label",>{ "Price" }</label>
                            <div class="control",>
                                <input class="input",
                                       type="text",
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
            </div>
        }
    }
}

