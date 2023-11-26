use super::setup::Setup;
use crate::{
    models::{candle::Candle, interval::Interval, strategy_orientation::StrategyOrientation},
    resolution_strategies::ResolutionStrategy,
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct SetupBuilder {
    pub candle: Option<Candle>,
    pub orientation: Option<StrategyOrientation>,
    pub ticker: Option<String>,
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
            ticker: None,
            interval: None,
            resolution_strategy: None,
            stop_loss: None,
            take_profit: None,
        }
    }

    pub fn candle(mut self, candle: Candle) -> Self {
        self.candle = Some(candle);
        self
    }

    pub fn orientation(mut self, orientation: StrategyOrientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn ticker(mut self, ticker: String) -> Self {
        self.ticker = Some(ticker);
        self
    }

    pub fn interval(mut self, interval: Interval) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn resolution_strategy(mut self, resolution_strategy: ResolutionStrategy) -> Self {
        self.resolution_strategy = Some(resolution_strategy);
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
        let ticker = self.ticker.clone().ok_or(anyhow!("Ticker is required."))?;
        let interval = self
            .interval
            .clone()
            .ok_or(anyhow!("Interval is required."))?;

        Ok(Setup {
            candle,
            orientation,
            ticker,
            interval,
            resolution_strategy: self.resolution_strategy.clone(),
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
        })
    }
}
