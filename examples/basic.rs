use jsonrpc_core::{
    IoHandler, Value,
};

fn main() {
    let mut io = IoHandler::new();

    io.add_sync_method("hello", |_| {
        Ok(Value::String("Hello, World!".to_owned()))
    });

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": [42, 23], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"Hello, World!","id":1}"#;

    assert_eq!(io.handle_request_sync(request), Some(response.to_owned()))
}