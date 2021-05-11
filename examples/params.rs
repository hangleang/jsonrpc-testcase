use jsonrpc_core::{
    IoHandler, Params, Value
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct MyParams {
    name: String
}

fn main() {
    let mut io = IoHandler::new();

    io.add_method("hello", |params: Params| async move {
        let parsed: MyParams = params.parse().unwrap();

        Ok(Value::String(format!("Hello, {}!", parsed.name)))
    });

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": { "name": "myname" }, "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"Hello, myname!","id":1}"#;

    assert_eq!(io.handle_request_sync(request), Some(response.to_owned()))
}