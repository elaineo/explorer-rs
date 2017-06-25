//! # Send HTTP RPC requests

use jsonrpc_core::{Error, Value};
use jsonrpc_core::futures::{BoxFuture, Future};
use reqwest::{Client, Url};

pub struct AsyncWrapper {
    url: Url,
}

impl AsyncWrapper {
    pub fn new(url: &String) -> AsyncWrapper {
        AsyncWrapper { url: Url::parse(url).expect("Expect to encode request url") }
    }

    pub fn request(&self, params: &::MethodParams) -> BoxFuture<Value, Error> {
        let client = Client::new().expect("Expect to create a request client");

        let mut res = client
            .post(self.url.clone())
            .json(params)
            .send()
            .expect("Failed to send request");

        let json: Value = res.json()
            .expect("Expect to deserialize a response as JSON");

        ::jsonrpc_core::futures::finished(json["result"].clone()).boxed()
    }
}