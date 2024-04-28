use actix::Addr;
use anyhow::{Result, anyhow};
use crate::{
    data_sources::datasource::DataSource, 
    resolution_strategies::resolution_strategy::ResolutionStrategy,
    models::{
        strategy_orientation::StrategyOrientation,
        timeseries::TimeSeries
    }
};

use super::trade::Trade;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct TradeBuilder {
    pub symbol: Option<String>,
    pub quantity: Option<f64>,
    pub dollar_value: Option<f64>,
    pub source: Option<DataSource>,
    pub notifications_enabled: bool,
    pub trading_enabled: bool,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub orientation: Option<StrategyOrientation>,
    pub timeseries: Option<Addr<TimeSeries>>
}

impl TradeBuilder {
    pub fn new() -> Self {
        TradeBuilder {
            symbol: None,
            quantity: None,
            dollar_value: None,
            source: None,
            notifications_enabled: false,
            trading_enabled: false,
            resolution_strategy: None,
            orientation: None,
            timeseries: None,
        }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    #[allow(dead_code)]
    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn dollar_value(mut self, dollar_value: f64) -> Self {
        self.dollar_value = Some(dollar_value);
        self
    }

    pub fn source(mut self, source: DataSource) -> Self {
        self.source = Some(source);
        self
    }

    pub fn notifications_enabled(mut self, notifications_enabled: bool) -> Self {
        self.notifications_enabled = notifications_enabled;
        self
    }

    pub fn trading_enabled(mut self, trading_enabled: bool) -> Self {
        self.trading_enabled = trading_enabled;
        self
    }

    pub fn resolution_strategy(mut self, resolution_strategy: ResolutionStrategy) -> Self {
        self.resolution_strategy = Some(resolution_strategy);
        self
    }

    pub fn orientation(mut self, orientation: StrategyOrientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn timeseries(mut self, timeseries: Addr<TimeSeries>) -> Self {
        self.timeseries = Some(timeseries);
        self
    }

    pub fn build(&self) -> Result<Trade> {
        let notifications_enabled = self.notifications_enabled;
        let trading_enabled = self.trading_enabled;
        let symbol = self.symbol.clone()
            .ok_or(anyhow!("Symbol is required to build Trade."))?;
        let quantity = self.quantity.clone()
            .ok_or(anyhow!("Quantity is required to build Trade."))?;
        let dollar_value = self.dollar_value.clone()
            .ok_or(anyhow!("Dollar value is required to build Trade."))?;
        let source = self.source.clone()
            .ok_or(anyhow!("DataSource is required to build Trade."))?;
        let resolution_strategy = self.resolution_strategy.clone()
            .ok_or(anyhow!("Resolution strategy is required to build Trade."))?;
        let orientation = self.orientation.clone()
            .ok_or(anyhow!("Orientation is required to build Trade."))?;
        let timeseries = self.timeseries.clone()
            .ok_or(anyhow!("TimeSeries is required to build Trade."))?;

        let trade = Trade {
            symbol,
            quantity,
            dollar_value,
            source,
            notifications_enabled,
            trading_enabled,
            resolution_strategy,
            orientation,
            timeseries
        };

        Ok(trade)
    }
}
