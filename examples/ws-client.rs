use jsonrpc_core::{Params, serde_json::json};
use jsonrpc_core_client::{RawClient, transports::ws};

#[tokio::main]
async fn main() {
    let client: RawClient = ws::connect(&"ws://127.0.0.1:3030".parse().unwrap()).await.unwrap();
    if let Ok(mut stream) = client.subscribe("subFav", Params::Array(vec![json!(27 as usize)]), "notifySub", "unsubFav") {
        if let Ok(val) = stream.try_next() {
            if let Some(val) = val {
                match val {
                    Ok(val) => println!("Response: {}", val),
                    Err(err) => eprintln!("Error: {}", err)
                }
            }
        }
    }
}