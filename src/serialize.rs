//! # Serialize RPC parameters in JSON

use jsonrpc_core::Params;

use serde::ser::{Serialize, Serializer};
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use ::MethodParams;
use ::Method;

static NONE: Params = Params::None;

static REQ_ID: AtomicUsize = ATOMIC_USIZE_INIT;

impl<'a> Serialize for MethodParams<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match self.0 {
            Method::EthBlockNumber => serializer.serialize_some(&method("eth_blockNumber")),
            Method::EthGetBalance => {
                serializer.serialize_some(&method_params("eth_getBalance", self.1))
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct JsonData<'a> {
    jsonrpc: &'static str,
    method: &'static str,
    params: &'a Params,
    id: usize,
}

fn method(method: &'static str) -> JsonData {
    method_params(method, &NONE)
}

fn method_params<'a>(method: &'static str, params: &'a Params) -> JsonData<'a> {
    let id = REQ_ID.fetch_add(1, Ordering::SeqCst);

    JsonData {
        jsonrpc: "2.0",
        method: method,
        params: params,
        id: id,
    }
}

#[cfg(test)]
mod tests {

    use super::method;

    #[test]
    fn should_increase_request_ids() {
        assert_eq!(method("").id, 1);
        assert_eq!(method("").id, 2);
        assert_eq!(method("").id, 3);
    }
}