use std::{collections::HashMap, sync::Arc};

use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    RwLock,
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

#[derive(Debug, Clone, Default)]
pub struct WsPool {
    inner: Arc<RwLock<HashMap<String, UnboundedSender<Message>>>>,
}

impl WsPool {
    pub fn new() -> Self {
        WsPool::default()
    }
}

impl WsPool {
    pub async fn insert_from_socket(&self, socket: WebSocket) {
        let (mut tx, mut rx) = socket.split();
        let (ttx, trx) = mpsc::unbounded_channel();

        let mut trx = UnboundedReceiverStream::new(trx);

        tokio::spawn(async move {
            while let Some(msg) = trx.next().await {
                if let Err(e) = tx.send(msg).await {
                    trace!("{:#?}", e)
                }
            }
        });

        let id = nanoid::nanoid!(16);
        info!("🚀 Client terhubung id: {}", &id);

        self.inner.write().await.insert(id.clone(), ttx);

        while (rx.next().await).is_some() {}

        info!("❌ Client terputus id: {}", &id);
        self.inner.write().await.remove(&id);
    }

    pub async fn send_to_all(&self, msg: impl Serialize) {
        let targets = self.inner.read().await;
        let message = serde_json::json!(msg).to_string();

        for (id, sender) in targets.clone().into_iter() {
            let msg = message.clone();
            info!("📨 Mengirim pesan socket ke client id: {}", id);
            tokio::spawn(async move { sender.send(Message::text(msg)) });
        }
    }
}
