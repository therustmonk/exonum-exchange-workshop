use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask};
use exonum::ExonumService;

pub struct Context {
    fetch: FetchService,
    exonum: ExonumService,
}

impl Context {
    pub fn new() -> Self {
        Self {
            fetch: FetchService::new(),
            exonum: ExonumService::new(),
        }
    }

    pub fn fetch(&mut self) -> &mut FetchService {
        &mut self.fetch
    }

    pub fn exonum(&mut self) -> &mut ExonumService {
        &mut self.exonum
    }

    pub fn fetch_updates(&mut self, callback: Callback<()>) -> FetchTask {
        unimplemented!();
    }

}
