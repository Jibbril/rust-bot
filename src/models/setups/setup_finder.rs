use actix::{Addr, Actor, Context, Handler};
use crate::{models::{traits::trading_strategy::TradingStrategy, timeseries::TimeSeries, interval::Interval, candle_added_payload::CandleAddedPayload}, data_sources::datasource::DataSource};

#[allow(dead_code)]
#[derive(Debug)]
pub struct SetupFinder {
    strategy: Box<dyn TradingStrategy>,
    interval: Interval,
    symbol: String, 
    data_source: DataSource,
    ts: Addr<TimeSeries>,
}

impl Actor for SetupFinder {
    type Context = Context<Self>;
}

impl Handler<CandleAddedPayload> for SetupFinder {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, _ctx: &mut Context<Self>) -> Self::Result {
        // TODO: Query time series for candles needed to check for setups
    }
}