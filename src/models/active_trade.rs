use actix::{Addr, Actor, Context};
use crate::{resolution_strategies::resolution_strategy::ResolutionStrategy, models::timeseries::TimeSeries};

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct ActiveTrade {
    symbol: String,
    quantity: f64,
    dollar_value: f64,
    notifications_enabled: bool,
    trading_enabled: bool,
    resolution_strategy: ResolutionStrategy,
    timeseries: Addr<TimeSeries>
}

impl Actor for ActiveTrade {
    type Context = Context<Self>;
}

