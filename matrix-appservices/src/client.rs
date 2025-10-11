use std::{fmt::Debug, sync::Arc};

use async_channel::Receiver;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{ sync::Mutex, task::JoinHandle };

use crate::types::{AppServiceConfig, Request};

/// Wrapper around [`matrix_sdk::Client`]
#[derive(Clone, Debug)]
pub struct Client<E: Clone + Debug + Serialize + DeserializeOwned = ()> {
    client: matrix_sdk::Client,
    server: Arc<Mutex<JoinHandle<crate::Result<()>>>>,
    requests: Receiver<Request>,
    config: AppServiceConfig<E>
}
