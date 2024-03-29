use crate::{
    models::{
        candle::Candle, interval::Interval, setups::setup::Setup,
        strategy_orientation::StrategyOrientation,
    },
    resolution_strategies::resolution_strategy::ResolutionStrategy,
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct SetupBuilder {
    pub candle: Option<Candle>,
    pub orientation: Option<StrategyOrientation>,
    pub symbol: Option<String>,
    pub interval: Option<Interval>,
    pub resolution_strategy: Option<ResolutionStrategy>,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
}

#[allow(dead_code)] // TODO: Remove once used
impl SetupBuilder {
    pub fn new() -> Self {
        SetupBuilder {
            candle: None,
            orientation: None,
            symbol: None,
            interval: None,
            resolution_strategy: None,
            stop_loss: None,
            take_profit: None,
        }
    }

    pub fn candle(mut self, candle: &Candle) -> Self {
        self.candle = Some(candle.clone());
        self
    }

    pub fn orientation(mut self, orientation: &StrategyOrientation) -> Self {
        self.orientation = Some(orientation.clone());
        self
    }

    pub fn symbol(mut self, symbol: &str) -> Self {
        self.symbol = Some(symbol.to_string());
        self
    }

    pub fn interval(mut self, interval: &Interval) -> Self {
        self.interval = Some(interval.clone());
        self
    }

    pub fn resolution_strategy(mut self, resolution_strategy: &ResolutionStrategy) -> Self {
        self.resolution_strategy = Some(resolution_strategy.clone());
        self
    }

    pub fn stop_loss(mut self, stop_loss: f64) -> Self {
        self.stop_loss = Some(stop_loss);
        self
    }

    pub fn take_profit(mut self, take_profit: f64) -> Self {
        self.take_profit = Some(take_profit);
        self
    }

    pub fn build(&self) -> Result<Setup> {
        let candle = self.candle.clone().ok_or(anyhow!("Candle is required."))?;
        let orientation = self
            .orientation
            .ok_or(anyhow!("Orientation is required."))?;
        let symbol = self.symbol.clone().ok_or(anyhow!("Symbol is required."))?;
        let interval = self
            .interval
            .clone()
            .ok_or(anyhow!("Interval is required."))?;

        Ok(Setup {
            candle,
            orientation,
            symbol,
            interval,
            resolution_strategy: self.resolution_strategy.clone(),
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
        })
    }
}
