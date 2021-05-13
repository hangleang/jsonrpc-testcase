use jsonrpc_http_server::jsonrpc_core::{IoHandler, Params, Result};
use jsonrpc_http_server::ServerBuilder;
use jsonrpc_derive::rpc;

#[rpc]
pub trait Rpc {
    #[rpc(name = "hello", params = "raw")]
    fn hello(&self, params: Params) -> Result<String>;

	#[rpc(name = "notify")]
    fn notify(&self, a: usize, b: usize);
}

struct MyRpc;
impl Rpc for MyRpc {
    fn hello(&self, params: Params) -> Result<String> {
        Ok(format!("Hello: {:?}", params))
    }

	fn notify(&self, a: usize, b: usize) {
		println!("Notify with {} & Then {}", a, b);
	}
}

fn main() {
	let mut io = IoHandler::default();
    io.extend_with(MyRpc.to_delegate());

	let server = ServerBuilder::new(io)
		.threads(3)
		.start_http(&"127.0.0.1:3030".parse().unwrap())
		.unwrap();

	server.wait();
}