use crate::{data_sources::datasource::DataSource, models::timeseries::TimeSeries};
use actix::Addr;
use anyhow::Result;

use super::websocket_payload::WebsocketPayload;

pub struct WebsocketClient {
    source: DataSource,
    observers: Vec<Addr<TimeSeries>>,
}

impl WebsocketClient {
    pub fn new(source: DataSource) -> Self {
        Self {
            source,
            observers: vec![],
        }
    }

    pub fn add_observer(&mut self, observer: Addr<TimeSeries>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&self, payload: WebsocketPayload) {
        for observer in &self.observers {
            observer.do_send(payload.clone());
        }
    }

    pub async fn connect(&self) -> Result<()> {
        self.source.connect_ws(self).await
    }
}
