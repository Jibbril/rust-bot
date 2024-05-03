use crate::{data_sources::datasource::DataSource, models::{setups::setup::Setup, strategy_orientation::StrategyOrientation, timeseries::TimeSeries}, resolution_strategies::resolution_strategy::ResolutionStrategy};
use actix::Addr;
use anyhow::{anyhow, Result};

use super::trade::Trade;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TradeBuilder {
    pub setup: Option<Setup>,
    pub symbol: Option<String>,
    pub quantity: Option<f64>,
    pub dollar_value: Option<f64>,
    pub source: Option<DataSource>,
    pub notifications_enabled: bool,
    pub trading_enabled: bool,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub orientation: Option<StrategyOrientation>,
    pub timeseries_addr: Option<Addr<TimeSeries>>,
}

impl TradeBuilder {
    pub fn new() -> Self {
        TradeBuilder {
            setup: None,
            symbol: None,
            quantity: None,
            dollar_value: None,
            source: None,
            notifications_enabled: false,
            trading_enabled: false,
            resolution_strategy: None,
            orientation: None,
            timeseries_addr: None,
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

    pub fn timeseries_addr(mut self, timeseries: Addr<TimeSeries>) -> Self {
        self.timeseries_addr = Some(timeseries);
        self
    }

    pub fn build(&self) -> Result<Trade> {
        let notifications_enabled = self.notifications_enabled;
        let trading_enabled = self.trading_enabled;
        let setup = self
            .setup
            .clone()
            .ok_or(anyhow!("Setup is required to build Trade."))?;
        let quantity = self
            .quantity
            .clone()
            .ok_or(anyhow!("Quantity is required to build Trade."))?;
        let dollar_value = self
            .dollar_value
            .clone()
            .ok_or(anyhow!("Dollar value is required to build Trade."))?;
        let source = self
            .source
            .clone()
            .ok_or(anyhow!("DataSource is required to build Trade."))?;
        let resolution_strategy = self
            .resolution_strategy
            .clone()
            .ok_or(anyhow!("Resolution strategy is required to build Trade."))?;
        let timeseries = self
            .timeseries_addr
            .clone()
            .ok_or(anyhow!("TimeSeries is required to build Trade."))?;

        let trade = Trade {
            setup,
            quantity,
            dollar_value,
            source,
            notifications_enabled,
            trading_enabled,
            resolution_strategy,
            timeseries,
        };

        Ok(trade)
    }
}
