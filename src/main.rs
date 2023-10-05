use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt, TryFutureExt};
use log::{error, info, warn};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;
use warp::Filter;

mod models;
use models::{Client, Message};
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();

    let clients = Clients::default();

    let route = warp::path!(String)
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .map(|identifier, ws: warp::ws::Ws, clients| {
            ws.on_upgrade(move |socket| client_connect(identifier, socket, clients))
        });

    info!("Listening on: {}", 1657); // TODO 从配置文件读取端口号等信息
    warp::serve(route).run(([0, 0, 0, 0], 1657)).await;
}

//包一下，让它可以克隆
fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn client_connect(identifier: String, ws: WebSocket, clients: Clients) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    warn!("websocket send error: {}", e);
                })
                .await;
        }
    });

    clients
        .write()
        .await
        .insert(identifier.clone(), Client::new(identifier.clone(), tx));
    info!("{} connected", identifier);
    info!(
        "connected clients' indetifier: {}",
        clients.read().await.len()
    );

    while let Some(result) = ws_rx.next().await {
        let ws_msg = match result {
            Ok(ws_msg) => ws_msg,
            Err(e) => {
                warn!("websocket error({}): {}", identifier, e);
                break;
            }
        };
        info!("{}: {:?}", identifier, ws_msg);
        if ws_msg.is_close() {
            break;
        }
        let msg = Message::from_json(ws_msg.to_str().unwrap());
        match msg {
            Ok(msg) => {
                client_send(msg, clients.clone()).await;
            }
            Err(e) => {
                warn!("json parsing error({}): {}", identifier, e);
            }
        }
    }

    client_disconnect(identifier, clients.clone()).await;
}

async fn client_disconnect(identifier: String, clients: Clients) {
    clients.write().await.remove(&identifier);
    warn!("{} disconnected", identifier);
}

async fn client_send(message: Message, clients: Clients) {
    let mut clients = clients.write().await;
    let client = match clients.get_mut(&message.target) {
        Some(client) => client,
        None => {
            error!("{} not found", message.target);
            return;
        }
    };
    client.tx.send(message.to_ws_message()).unwrap();
}
