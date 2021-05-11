use jsonrpc_core::{Params, Value};
use jsonrpc_core_client::{RawClient, transports::ws};

#[tokio::main]
async fn main() {
    let client: RawClient = ws::connect(&"ws://127.0.0.1:3030".parse().unwrap()).await.unwrap();
    let fut = client.call_method("sub_meal", Params::Array(vec![Value::Number(27.into())]));

    match fut.await {
        Ok(val) => println!("Response: {}", val),
        Err(err) => eprintln!("Error: {}", err)
    }
}