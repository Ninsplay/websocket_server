use tokio::sync::mpsc::UnboundedSender;
use warp::ws::Message;

pub struct Client {
    pub identifier: String,
    pub tx: UnboundedSender<Message>,
}

impl Client {
    pub fn new(identifier: String, tx: UnboundedSender<Message>) -> Self {
        Self { identifier, tx }
    }
}
