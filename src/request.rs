//! # Send HTTP RPC requests

use hyper::Url;
use hyper::client::IntoUrl;
use jsonrpc_core::{Error, Value};
use jsonrpc_core::futures::{BoxFuture, Future};
use jsonrpc_core::futures;
use reqwest::Client;

use connect::MethodParams;

pub struct AsyncWrapper {
    pub url: Url,
}

impl AsyncWrapper {
    pub fn new<U: IntoUrl>(url: U) -> AsyncWrapper {
        AsyncWrapper { url: url.into_url().expect("Expect to encode request url") }
    }

    pub fn request(&self, params: &MethodParams) -> BoxFuture<Value, Error> {
        let client = Client::new().expect("Expect to create a request client");

        let mut res = client
            .post(self.url.clone())
            .json(params)
            .send()
            .expect("Expect to receive response");

        res
        //let json = res.json()
        //    .expect("Expect to deserialize a response as JSON");

        //::jsonrpc_core::futures::finished(json["result"].clone()).boxed()
    }
}