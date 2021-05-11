use jsonrpc_core::{
    MetaIoHandler, Value,
};
use jsonrpc_ipc_server::ServerBuilder;

fn main() {
    let mut io = MetaIoHandler::<()>::default();
    io.add_sync_method("hello", |_| {
        Ok(Value::String("Hello, world!".to_owned()))
    });

    ServerBuilder::new(io).start("/tmp/hello.ipc")
    .expect("Failed to start IPC server")
    .wait()
}