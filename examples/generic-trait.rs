use jsonrpc_core::{
    IoHandler, Result, BoxFuture,
    futures,
};
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc<One, Two> {
    #[rpc(name = "getOne")]
    fn one(&self) -> Result<One>;

    #[rpc(name = "setTwo")]
    fn two(&self, x: Two) -> Result<()>;

    #[rpc(name = "callAsync")]
    fn call(&self, x: One) -> BoxFuture<Result<(One, Two)>>;
}

struct MyRpc;
impl Rpc<usize, String> for MyRpc {
    fn one(&self) -> Result<usize> {
        Ok(100)
    }

    fn two(&self, x: String) -> Result<()> {
        println!("set value to {}", x);
        Ok(())
    }

    fn call(&self, x: usize) -> BoxFuture<Result<(usize, String)>> {
        Box::pin(futures::future::ready(Ok((x, format!("x: {}", x)))))
    }
}

fn main() {
    let mut io = IoHandler::new();
    io.extend_with(MyRpc.to_delegate())
}