use crate::models::{
    candle::Candle, interval::Interval, setups::setup::Setup,
    strategy_orientation::StrategyOrientation,
};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
pub struct SetupBuilder {
    pub candle: Option<Candle>,
    pub orientation: Option<StrategyOrientation>,
    pub symbol: Option<String>,
    pub interval: Option<Interval>,
}

#[allow(dead_code)] // TODO: Remove once used
impl SetupBuilder {
    pub fn new() -> Self {
        SetupBuilder {
            candle: None,
            orientation: None,
            symbol: None,
            interval: None,
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
        })
    }
}
