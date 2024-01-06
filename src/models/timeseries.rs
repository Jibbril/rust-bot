use super::{
    candle::Candle,
    interval::Interval,
    message_payloads::{
        latest_candles_payload::LatestCandleResponse,
        request_latest_candles_payload::RequestLatestCandlesPayload,
        ts_subscribe_payload::TSSubscribePayload, websocket_payload::WebsocketPayload,
    },
    setups::setup_finder::SetupFinder,
};
use crate::{
    data_sources::{datasource::DataSource, local},
    indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf},
    models::message_payloads::candle_added_payload::CandleAddedPayload,
};
use actix::{Actor, Addr, Context, Handler};
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use std::{collections::HashSet, ops::Add};

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub max_length: usize,
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

impl Handler<RequestLatestCandlesPayload> for TimeSeries {
    type Result = Result<LatestCandleResponse>;

    fn handle(
        &mut self,
        msg: RequestLatestCandlesPayload,
        _ctx: &mut Context<Self>,
    ) -> Self::Result {
        let candles = if self.candles.len() < msg.n {
            // Return what's available
            self.candles.clone()
        } else {
            // Return requested number of candles
            self.candles[self.candles.len() - msg.n..].to_vec()
        };

        Ok(LatestCandleResponse {
            symbol: self.ticker.clone(),
            interval: self.interval.clone(),
            candles,
        })
    }
}

impl Handler<TSSubscribePayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: TSSubscribePayload, _ctx: &mut Context<Self>) -> Self::Result {
        let observer = msg.observer;
        self.observers.push(observer);
    }
}

impl TimeSeries {
    pub fn new(ticker: String, interval: Interval, candles: Vec<Candle>) -> Self {
        TimeSeries {
            ticker,
            interval,
            candles,
            max_length: 800, // Default value
            indicators: HashSet::new(),
            observers: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
    }

    fn validate_timeseries_integrity(&mut self, new_candle: DateTime<Utc>) -> Result<()> {
        // No need for validation if timeseries is empty
        if self.candles.len() == 0 { return Ok(()) };

        let last_candle = self.candles.last().expect("Expected more than 0 candles");
        let diff = new_candle.signed_duration_since(last_candle.timestamp);

        let step = self.interval.to_duration();
        let delta = self.interval.max_diff();

        // New is subsequent candle so timeseries integrity ok
        if diff >= step - delta && diff < step + delta { return Ok(())}

        println!("GAP at: {}", new_candle);

        // TODO: Add proper logic to fetch candle data below instead of just filling 
        // with previous value

        let new_timestamp = new_candle.clone().add(-step);

        if new_timestamp < last_candle.timestamp {
            return Err(anyhow!("Added candle has timestamp earlier in time than current last candle."));
        }

        let next = Candle::from_val(new_timestamp, last_candle.close, 0.0);
        self.add_candle(next)?;

        Ok(())
    } 

    pub fn add_candle(&mut self, candle: Candle) -> Result<()> {
        self.validate_timeseries_integrity(candle.timestamp)?;
        self.candles.push(candle.clone());

        let indicator_types = self.indicators.clone();
        for indicator_type in indicator_types {
            indicator_type.populate_last_candle(self)?;
        }

        println!("Added candle: {:#?}", self.candles.last());

        // Notify observers
        let payload = CandleAddedPayload { candle };

        for observer in &self.observers {
            observer.do_send(payload.clone());
        }

        // Remove oldest candle if max length is exceeded
        if self.candles.len() > self.max_length {
            self.candles.remove(0);
        }

        Ok(())
    }

    pub fn set_candles(&mut self, candles: &[Candle]) {
        self.candles = candles.to_vec();
    }

    pub fn get_candles(&self) -> Vec<Candle> {
        self.candles.clone()
    }

    pub fn clear_candles(&mut self) {
        self.candles.clear()
    }

    pub fn add_indicator(&mut self, indicator_type: IndicatorType) -> Result<()> {
        if self.indicators.contains(&indicator_type) {
            return Ok(());
        }

        indicator_type.populate_candles(self)?;

        self.indicators.insert(indicator_type);

        Ok(())
    }

    pub fn dummy() -> Self {
        Self::new("DUMMY".to_string(), Interval::Day1, Vec::new())
    }

    #[allow(dead_code)]
    pub async fn save_to_local(&self, source: &DataSource) -> Result<()> {
        local::write(self, source).await
    }
}
