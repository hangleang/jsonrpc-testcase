use jsonrpc_core::Params;
use jsonrpc_core_client::{
    RawClient, transports::ipc
};

#[tokio::main]
async fn main() {
    let client: RawClient = ipc::connect("/tmp/hello.ipc").await.unwrap();
    let fut = client.call_method("hello", Params::None);
    
    match fut.await {
        Ok(val) => println!("Response: {}", val),
        Err(err) => eprintln!("Error: {}", err)
    }
}