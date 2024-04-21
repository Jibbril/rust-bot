use actix::Addr;
use anyhow::{Result, Context};

use crate::models::{
    traits::trading_strategy::TradingStrategy, 
    timeseries::TimeSeries,
    setups::setup_finder::SetupFinder, active_trade::ActiveTrade
};

pub struct SetupFinderBuilder {
    strategy: Option<Box<dyn TradingStrategy>>,
    ts: Option<Addr<TimeSeries>>,
    notifications_enabled: bool,
    live_trading_enabled: bool,
    spawned_trades: Vec<Addr<ActiveTrade>>
}

impl SetupFinderBuilder {
    pub fn new() -> Self {
        SetupFinderBuilder {
            strategy: None,
            ts: None,
            notifications_enabled: false,
            live_trading_enabled: false,
            spawned_trades: vec![]
        }
    }

    pub fn strategy(mut self, strategy: Box<dyn TradingStrategy>) -> Self {
        self.strategy = Some(strategy);
        self
    }

    pub fn ts(mut self, ts: Addr<TimeSeries>) -> Self {
        self.ts = Some(ts);
        self
    }

    pub fn notifications_enabled(mut self, enabled: bool) -> Self {
        self.notifications_enabled = enabled;
        self
    }

    pub fn live_trading_enabled(mut self, enabled: bool) -> Self {
        self.live_trading_enabled = enabled;
        self
    }

    pub fn spawned_trades(mut self, trades: &[Addr<ActiveTrade>]) -> Self {
        self.spawned_trades = trades.to_vec();
        self
    }

    pub fn build(self) -> Result<SetupFinder> {
        let strategy = self.strategy.context("Strategy is required to build SetupFinder")?;
        let ts = self.ts.context("TimeSeries address is required to build SetupFinder")?;
        let notifications_enabled = self.notifications_enabled;
        let live_trading_enabled = self.live_trading_enabled;
        let spawned_trades = self.spawned_trades;

        Ok(SetupFinder::new(
            strategy, 
            ts, 
            notifications_enabled, 
            live_trading_enabled,
            &spawned_trades
        )?)
    }
}
