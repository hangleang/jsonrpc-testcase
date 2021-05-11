use jsonrpc_core::{Params, Value};
use jsonrpc_core_client::{
    RawClient, transports::http
};

#[tokio::main]
async fn main() {
    let client: RawClient = http::connect("http://localhost:3030").await.unwrap();
    let fut = client.call_method("hello", Params::Array(vec![Value::String("World!".to_owned())]));
    
    match fut.await {
        Ok(val) => println!("Response: {}", val),
        Err(err) => eprintln!("Error: {}", err)
    }
}