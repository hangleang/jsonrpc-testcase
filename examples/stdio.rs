use jsonrpc_core::{
    IoHandler, Value,
};
use jsonrpc_stdio_server::ServerBuilder;

#[tokio::main]
async fn main() {
    let mut io = IoHandler::new();
    io.add_sync_method("hello", |_| {
        Ok(Value::String("Hello, world!".to_owned()))
    });

    ServerBuilder::new(io).build().await
}