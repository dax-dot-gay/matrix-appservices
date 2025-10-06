use std::{ net::SocketAddr, sync::Arc };

use async_channel::Receiver;
use tokio::{ sync::Mutex, task::JoinHandle };

use crate::types::Request;

/// Wrapper around [`matrix_sdk::Client`]
#[derive(Clone, Debug)]
pub struct Client {
    client: matrix_sdk::Client,
    service_bind: SocketAddr,
    server: Arc<Mutex<JoinHandle<crate::Result<()>>>>,
    requests: Receiver<Request>,
}
