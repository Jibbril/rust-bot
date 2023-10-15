use anyhow::Result;
use super::websocketpayload::WebsocketPayload;
use crate::data_sources::{bitfinex, bybit, DataSource};

pub struct WebsocketClient {
    source: DataSource,
    observers: Vec<u64>,
}

impl WebsocketClient {
    pub fn new(source: DataSource) -> Self {
        Self {
            source,
            observers: vec![]
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
