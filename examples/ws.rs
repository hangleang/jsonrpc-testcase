use std::thread;
use std::sync::{atomic, Arc, RwLock};
use std::collections::HashMap;
use jsonrpc_core::{Result, Error, ErrorCode};
use jsonrpc_ws_server::{ServerBuilder, RequestContext};
use jsonrpc_pubsub::{
    typed, PubSubHandler, Session, SubscriptionId
};
use jsonrpc_derive::rpc;

#[rpc]
pub trait RpcPubSub {
    type Metadata;

    #[pubsub(subscription = "fav", subscribe, name = "subFav")]
    fn subscribe(&self, meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: usize);

    #[pubsub(subscription = "fav", unsubscribe, name = "unsubFav")]
    fn unsubscribe(&self, meta: Option<Self::Metadata>, sub_id: SubscriptionId) -> Result<bool>;

    #[rpc(name = "notifySub")]
    fn notify_sub(&self);
}

#[derive(Debug, Default)]
struct MyRpcPubSub {
    uid: atomic::AtomicUsize,
    map: Arc<RwLock<HashMap<SubscriptionId, typed::Sink<String>>>>,
}

impl RpcPubSub for MyRpcPubSub {
    type Metadata = Arc<Session>;

    fn subscribe(&self, _meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: usize) {
        if param != 27 {
            subscriber.reject(Error {
                code: ErrorCode::InvalidParams,
                message: format!("Invalid params: {}", param),
                data: None
            }).unwrap()
        } else {
            let id = self.uid.fetch_add(1, atomic::Ordering::SeqCst);
            let sub_id = SubscriptionId::Number(id as u64);
            if let Ok(sink) = subscriber.assign_id(sub_id.clone()) {
                if let Ok(mut map) = self.map.write() {
                    map.insert(sub_id, sink);
                }
            }
        }
    }

    fn unsubscribe(&self, _meta: Option<Self::Metadata>, sub_id: SubscriptionId) -> Result<bool> {
        let mut res = false;
        if let Ok(mut map) = self.map.write() {
            if map.remove(&sub_id).is_some() {
                res = true;
            }
        }

        if res {
            Ok(res)
        } else {
            Err(Error {
                code: ErrorCode::InvalidRequest,
                message: format!("Invalid request/ Not found subscription id: {:?}", sub_id),
                data: None
            })
        }
    }

    fn notify_sub(&self) {
        println!("There is a subscription now!");
    }
}

fn main() {
    env_logger::init();

    let mut pubsub = PubSubHandler::default();
    let my_rpc = MyRpcPubSub::default();
    let mapping = my_rpc.map.clone();

    thread::spawn(move || loop {
        let subscriptions = mapping.read().unwrap();
        for sink in subscriptions.values() {
            let _ = sink.notify(Ok("Hello, world!".to_owned()));
        }
        thread::sleep(std::time::Duration::from_millis(1000))
    });

    pubsub.extend_with(my_rpc.to_delegate());

    let server = ServerBuilder::with_meta_extractor(pubsub, |ctx: &RequestContext| {
        Arc::new(Session::new(ctx.sender()))
    })
    .start(&"127.0.0.1:3030".parse().unwrap())
    .expect("Failed to start server");

    server.wait().unwrap();
}