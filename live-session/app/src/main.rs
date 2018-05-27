#[macro_use]
extern crate log;
extern crate web_logger;
extern crate yew;
extern crate app;

use yew::prelude::*;
use app::Model;

fn main() {
    web_logger::init();
    yew::initialize();
    info!("RustFest TRADING!");
    let context = ();
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
