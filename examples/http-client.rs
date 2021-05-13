use jsonrpc_core::{Params, Value, serde_json::json};
use jsonrpc_core_client::{
    RawClient, transports::http
};

#[tokio::main]
async fn main() {
    let client: RawClient = http::connect("http://localhost:3030").await.unwrap();
    let fut = client.call_method("hello", Params::Array(vec![Value::String("World!".to_owned())]));
    let _ = client.notify("notify", Params::Array(vec![json!(12), json!(27)]));
    
    match fut.await {
        Ok(val) => println!("Response: {}", val),
        Err(err) => eprintln!("Error: {}", err)
    }
}