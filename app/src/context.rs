use failure::Error;
use serde::Deserialize;
use std::time::Duration;
use yew::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::interval::{
    IntervalService,
    IntervalTask,
};
use yew::services::fetch::{
    FetchService,
    FetchTask,
    Request,
    Response,
};
use exonum::ExonumService;

#[derive(Deserialize, Debug)]
pub struct Account {
    pub owner: String,
    pub usd_balance: u64,
    pub token_balance: u64,
}

#[derive(Deserialize)]
pub struct Order {
    price: u64,
    amount: u64,
}

pub type OrderBook = Vec<Order>;

pub struct Context {
    interval: IntervalService,
    fetch: FetchService,
    exonum: ExonumService,
}

impl Context {
    pub fn new() -> Self {
        Self {
            interval: IntervalService::new(),
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

    pub fn schedule_updates(&mut self, callback: Callback<()>) -> IntervalTask {
        let duration = Duration::from_millis(300);
        self.interval.spawn(duration, callback)
    }

    pub fn fetch_account(&mut self, callback: Callback<Result<Account, String>>) -> FetchTask {
        let url = format!(
            "http://localhost:8080/api/services/cryptoexchange/v1/account/{}",
            self.exonum.get_owner());
        warn!("URL: {}", url);
        self.fetch_resource(url, callback)
    }

    fn fetch_resource<T>(&mut self, url: String, callback: Callback<Result<T, String>>) -> FetchTask
    where
        T: for <'de> Deserialize<'de> + 'static,
    {
        let request = Request::get(url)
            .body(Nothing)
            .unwrap();
        let callback = callback.reform(|response: Response<Json<Result<T, Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if meta.status.is_success() {
                data.map_err(|e| e.to_string())
            } else {
                Err("fetch error...".to_string())
            }
        });
        self.fetch.fetch(request, callback)
    }

}
