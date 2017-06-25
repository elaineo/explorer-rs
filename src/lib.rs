#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate reqwest;
extern crate rustc_serialize;

mod request;
mod serialize;

use jsonrpc_core::{Error, ErrorCode, IoHandler, Params};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};

use std::net::SocketAddr;
use std::sync::Arc;

/// RPC methods
pub enum Method {
    /// [eth_newBlockFilter](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_newblockfilter)
    EthNewBlockFilter,
    /// [eth_getBlockByNumber](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_getblockbynumber)
    EthGetBlockByNumber,
    /// [eth_blockNumber](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_blocknumber)
    EthBlockNumber,

    /// [eth_getBalance](https://github.com/ethereum/wiki/wiki/JSON-RPC#eth_getbalance)
    EthGetBalance,

}

/// RPC method's parameters
pub struct MethodParams<'a>(pub Method, pub &'a Params);

/// Start an HTTP RPC endpoint
pub fn start(addr: &SocketAddr, client_addr: &SocketAddr) {
    let mut io = IoHandler::default();

    let url = Arc::new(request::AsyncWrapper::new(&format!("http://{}", client_addr)));

    let params = Params::None;
    url.request(&MethodParams(Method::EthBlockNumber, &params));

    {
        let url = url.clone();

        io.add_async_method("eth_blockNumber",
                            move |p| url.request(&MethodParams(Method::EthBlockNumber, &p)));
    }


    {
        let url = url.clone();

        io.add_async_method("eth_getBalance",
                            move |p| url.request(&MethodParams(Method::EthGetBalance, &p)));
    }


    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Any,
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(addr)
        .expect("Expect to build HTTP RPC server");


    println!("Connector started on http://{}", server.address());

    server.wait();
}