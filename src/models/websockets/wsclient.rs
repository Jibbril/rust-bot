use std::sync::{Arc, Mutex};
use anyhow::Result;

use crate::data_sources::{DataSource,bitfinex,bybit};
use super::{observer::Observer, websocketpayload::WebsocketPayload, subject::Subject};

pub struct WebsocketClient {
    source: DataSource,
    observers: Arc<Mutex<Vec<Box<dyn Observer<WebsocketPayload> + Send>>>>,
}

impl Subject<WebsocketPayload> for WebsocketClient {
    fn add_observer(&mut self, observer: Box<dyn Observer<WebsocketPayload> + Send>) {
        self.observers.lock().unwrap().push(observer);
    }

    fn notify_observers(&self, value: WebsocketPayload) {
        for observer in self.observers.lock().unwrap().iter_mut() {
            observer.update(value.clone());
        }
    }
}

impl WebsocketClient {
    pub fn new(source: DataSource) -> Self {
        Self {
            source,
            observers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn listen(&self) -> Result<()> {
        match self.source {
            DataSource::Bitfinex => {
                bitfinex::ws::connect_ws(&self).await?;
            } 
            DataSource::Bybit => {
                bybit::ws::connect_ws(&self).await?;               
            }
            _ => panic!("Error"),
        }

        Ok(())
    }
}