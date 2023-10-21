use actix::Addr;
use anyhow::Result;
use crate::{data_sources::{bitfinex, bybit, datasource::DataSource}, models::timeseries::TimeSeries};

use super::websocket_payload::WebsocketPayload;

pub struct WebsocketClient {
    source: DataSource,
    observers: Vec<Addr<TimeSeries>>,
}

impl WebsocketClient {
    pub fn new(source: DataSource) -> Self {
        Self {
            source,
            observers: vec![]
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
