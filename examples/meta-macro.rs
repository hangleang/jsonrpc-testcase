use std::collections::BTreeMap;
use jsonrpc_core::{
    MetaIoHandler, Metadata, Params, Value, Result, BoxFuture,
    futures::future,
};
use jsonrpc_derive::rpc;
use jsonrpc_tcp_server::{ServerBuilder, RequestContext};

#[derive(Debug, Clone)]
struct Meta(String);
impl Metadata for Meta {}

#[rpc]
pub trait Rpc<One> {
    type Metadata;

    #[rpc(name = "getOne")]
    fn one(&self) -> Result<One>;

    #[rpc(name = "add")]
    fn add(&self, a: usize, b: usize) -> Result<usize>;

    #[rpc(name = "mul")]
    fn mul(&self, a: usize, b: Option<usize>) -> Result<usize>;

    #[rpc(name = "raw", params = "raw")]
    fn raw(&self, params: Params) -> Result<String>;

    #[rpc(name = "callAsync")]
    fn call(&self, a: usize) -> BoxFuture<Result<String>>;

    #[rpc(name = "callAsyncMeta", meta)]
    fn call_meta(&self, a: Self::Metadata, b: BTreeMap<String, Value>) -> BoxFuture<Result<String>>;

    #[rpc(name = "notify")]
    fn notify(&self, a: usize);
}

struct MyRpc;
impl Rpc<f32> for MyRpc {
    type Metadata = Meta;

    fn one(&self) -> Result<f32> {
        Ok(12.27)
    }

    fn add(&self, a: usize, b: usize) -> Result<usize> {
        Ok(a + b)
    }

    fn mul(&self, a: usize, b: Option<usize>) -> Result<usize> {
        let b = b.unwrap_or(1);
        Ok(a * b)
    }

    fn raw(&self, params: Params) -> Result<String> {
        Ok(format!("got: {:?}", params))
    }

    fn call(&self, x: usize) -> BoxFuture<Result<String>> {
        Box::pin(future::ready(Ok(format!("call: {}", x))))
    }

    fn call_meta(&self, meta: Self::Metadata, map: BTreeMap<String, Value>) -> BoxFuture<Result<String>> {
        Box::pin(future::ready(Ok(format!("From: {}, got: {:?}", meta.0, map))))
    }

    fn notify(&self, a: usize) {
        println!("Received notify with value: {}", a)
    }
}

fn main() {
    let mut io = MetaIoHandler::default();
    io.extend_with(MyRpc.to_delegate());

    let server = ServerBuilder::with_meta_extractor(io, |context: &RequestContext| {
        Meta(format!("{}", context.peer_addr))
    })
    .start(&"0.0.0.0:3030".parse().unwrap())
    .expect("failed to start server");

    server.wait();
}