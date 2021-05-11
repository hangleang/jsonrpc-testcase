use jsonrpc_core::{
    IoHandler, Result, BoxFuture,
    futures::{self, TryFutureExt},
};
use jsonrpc_core_client::transports::local;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
    #[rpc(name = "protocolVersion")]
    fn protocol_version(&self) -> Result<String>;

    #[rpc(name = "add")]
    fn add(&self, a: usize, b: usize) -> Result<usize>;

    #[rpc(name = "callAsync")]
    fn call_async(&self, a: usize) -> BoxFuture<Result<String>>;
}

pub struct MyRpc;
impl Rpc for MyRpc {
    fn protocol_version(&self) -> Result<String> {
        Ok("1.0".to_owned())
    }

    fn add(&self, a: usize, b: usize) -> Result<usize> {
        Ok(a + b)
    }

    fn call_async(&self, a: usize) -> BoxFuture<Result<String>> {
        Box::pin(futures::future::ready(Ok(format!("Call of {}", a))))
    }
}

fn main() {
    let mut io = IoHandler::new();
    io.extend_with(MyRpc.to_delegate());
    println!("Starting local server");
    let (client, server) = local::connect::<RpcClient, _, _>(io);
    let fut = client.add(5, 6).map_ok(|res| println!("result after add: {}", res));

    futures::executor::block_on(async move {
        futures::join!(fut, server)
    }).0.unwrap()
}