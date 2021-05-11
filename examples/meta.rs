use jsonrpc_core::{
    Metadata, MetaIoHandler, Value, Params
};

#[derive(Debug, Clone)]
struct Meta(usize);
impl Metadata for Meta {}

fn main() {
    let mut io = MetaIoHandler::default();

    io.add_method_with_meta("hello", |_: Params, meta: Meta| async move {
        Ok(Value::String(format!("Hello, {}!", meta.0)))
    });

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": [42, 23], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"Hello, 27!","id":1}"#;

    assert_eq!(io.handle_request_sync(request, Meta(27)), Some(response.to_owned()))
}