use std::collections::HashMap;
use std::sync::{atomic, Arc, RwLock};
use std::thread;
use jsonrpc_core::{
    Error, ErrorCode, Result
};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::{
    typed, PubSubHandler, Session, SubscriptionId
};
use jsonrpc_tcp_server::{ServerBuilder, RequestContext};

#[rpc]
pub trait Rpc {
    type Metadata;

    #[pubsub(subscription = "mine", subscribe, name = "mysubscribe", alias("mysub"))]
    fn subscribe(&self, meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: usize);

    #[pubsub(subscription = "mine", unsubscribe, name = "myunsubscribe", alias("myunsub"))]
    fn unsubscribe(&self, meta: Option<Self::Metadata>, subscription: SubscriptionId) -> Result<bool>;
}

#[derive(Debug, Default)]
struct MyRpc {
    uid: atomic::AtomicUsize,
    map: Arc<RwLock<HashMap<SubscriptionId, typed::Sink<String>>>>,
}
impl Rpc for MyRpc {
    type Metadata = Arc<Session>;

    fn subscribe(&self, _meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: usize) {
        if param != 12 {
            subscriber.reject(Error {
                code: ErrorCode::InvalidParams,
                message: format!("Invalid Param: {}", param),
                data: None
            }).unwrap()
        } else {
            let id = self.uid.fetch_add(1, atomic::Ordering::SeqCst);
            let sub_id = SubscriptionId::Number(id as u64);
            let sink = subscriber.assign_id(sub_id.clone()).unwrap();
            self.map.write().unwrap().insert(sub_id, sink);
        }
    }

    fn unsubscribe(&self, _meta: Option<Self::Metadata>, sub_id: SubscriptionId) -> Result<bool> {
        let rem = self.map.write().unwrap().remove(&sub_id);

        if rem.is_some() {
            Ok(true)
        } else {
            Err(Error {
                code: ErrorCode::InvalidParams,
                message: format!("Invalid subscription: {:?}", sub_id),
                data: None
            })
        }
    }
}

fn main() {
    let mut pubsub = PubSubHandler::default();
    let my_rpc = MyRpc::default();
    let mapping = my_rpc.map.clone();

    thread::spawn(move || loop {
        let subscriptions = mapping.read().unwrap();
        for sink in subscriptions.values() {
            let _ = sink.notify(Ok("Hello, world!".to_owned()));
        }
        thread::sleep(std::time::Duration::from_millis(1000))
    });

    pubsub.extend_with(my_rpc.to_delegate());

    let server = ServerBuilder::with_meta_extractor(pubsub, |context: &RequestContext| {
        Arc::new(Session::new(context.sender.clone()))
    })
    .start(&"127.0.0.1:3030".parse().unwrap())
    .expect("Failed to start server");

    server.wait()
}