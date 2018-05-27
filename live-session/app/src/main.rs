#[macro_use]
extern crate log;
extern crate web_logger;
extern crate yew;

fn main() {
    web_logger::init();
    yew::initialize();
    info!("RustFest TRADING!");
    yew::run_loop();
}
