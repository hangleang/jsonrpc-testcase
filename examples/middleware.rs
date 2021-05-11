use jsonrpc_core::{
    MetaIoHandler, Value, Metadata, Middleware, Request, Response, FutureResponse,
    middleware::NoopCallFuture,
    
};
use futures_util::{
    future::Either, FutureExt
};
use std::future::Future;
use std::time::Instant;
use std::sync::atomic::{self, AtomicUsize};

#[derive(Debug, Clone)]
struct Meta(usize);
impl Metadata for Meta {}

#[derive(Default)]
struct MyMiddleware(AtomicUsize);
impl Middleware<Meta> for MyMiddleware {
    type Future = FutureResponse;
    type CallFuture = NoopCallFuture;

    fn on_request<F, X>(&self, request: Request, meta: Meta, next: F) -> Either<Self::Future, X>
    where 
        F: FnOnce(Request, Meta) -> X + Send,
        X: Future<Output = Option<Response>> + Send + 'static,
    {
        let start = Instant::now();
        let req_time = self.0.fetch_add(1, atomic::Ordering::SeqCst);
        println!("Processing request {}: {:?}, {:?}", req_time, request, meta);

        Either::Left(Box::pin(next(request, meta).map(move |res| {
            println!("Precessing took: {:?}", start.elapsed());
            res
        })))
    }
}

fn main() {
    let mut io = MetaIoHandler::with_middleware(MyMiddleware::default());

    io.add_method_with_meta("hello", |_, meta: Meta| async move {
        Ok(Value::String(format!("Hello, {}!", meta.0)))
    });

    let request = r#"{"jsonrpc": "2.0", "method": "hello", "params": [42, 23], "id": 1}"#;
	let response = r#"{"jsonrpc":"2.0","result":"Hello, 27!","id":1}"#;

    assert_eq!(io.handle_request_sync(request, Meta(27)), Some(response.to_owned()))
}