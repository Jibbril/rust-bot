use actix::{Actor, Context, Handler, Addr};
use anyhow::Result;

use super::{candle::Candle, interval::Interval, setups::setup_finder::SetupFinder, message_payloads::websocket_payload::WebsocketPayload};
use crate::{
    data_sources::{datasource::DataSource, local},
    indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf},
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
    pub indicators: HashSet<IndicatorType>,
    pub observers: Vec<Addr<SetupFinder>>,
}

impl Actor for TimeSeries {
    type Context = Context<Self>;
}

impl Handler<WebsocketPayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: WebsocketPayload, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.ok {
            if let Some(candle) = msg.candle {
                match self.add_candle(candle) {
                    Ok(_) => (),
                    Err(e) => {
                        // TODO: Error handling
                        println!("Error adding candle: {:#?}", e)
                    }
                };
            }
        } else {
            println!(
                "Error: {}",
                match msg.message {
                    Some(message) => message,
                    None => "Unknown error".to_string(),
                }
            );
        }
    }
}

impl TimeSeries {
    pub fn new(ticker: String, interval: Interval, candles: Vec<Candle>) -> Self {
        TimeSeries {
            ticker,
            interval,
            candles,
            indicators: HashSet::new(),
            observers: vec![],
        }
    }

    fn add_candle(&mut self, candle: Candle) -> Result<()> {
        // TODO: Perform checks of timestamp to ensure that no
        // duplicates are added, or that there has not been any
        // missed candles in between.
        self.candles.push(candle);

        let indicator_types = self.indicators.clone();
        for indicator_type in indicator_types {
            indicator_type.populate_last_candle(self)?;
        }

        println!("Added candle: {:#?}", self.candles.last());

        Ok(())
    }

    pub fn dummy() -> Self {
        Self::new(
            "DUMMY".to_string(),
            Interval::Day1,
            Vec::new(),
        )
    }

    #[allow(dead_code)]
    pub async fn save_to_local(&self, source: &DataSource) -> Result<()> {
        local::write(self, source).await
    }
}
