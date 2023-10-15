use actix::{Actor, Context, Handler};

use super::{
    candle::Candle,
    interval::Interval, websockets::websocket_payload::WebsocketPayload,
};
use crate::indicators::indicator_type::IndicatorType;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct TimeSeries {
    pub ticker: String,
    pub interval: Interval,
    pub candles: Vec<Candle>,
    pub indicators: HashSet<IndicatorType>,
}

impl Actor for TimeSeries {
    type Context = Context<Self>;
}

impl Handler<WebsocketPayload> for TimeSeries {
    type Result = ();

    fn handle(&mut self, msg: WebsocketPayload, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.ok {
            if let Some(candle) = msg.candle {
                println!("Adding Candle:{:#?}", candle.clone());
                self.add_candle(candle);
            }
        } else {
            println!("Error: {}", match msg.message {
                Some(message) => message,
                None => "Unknown error".to_string(),
            });
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
        }
    }

    fn add_candle(&mut self, candle: Candle) {
        // TODO: Perform checks of timestamp to ensure that no
        // duplicates are added, or that there has not been any
        // missed candles in between.
        self.candles.push(candle);
    }

    pub fn dummy() -> Self {
        TimeSeries {
            ticker: "DUMMY".to_string(),
            interval: Interval::Day1,
            candles: Vec::new(),
            indicators: HashSet::new(),
        }
    }
}
