use actix::Addr;
use anyhow::{Context, Result};
use crate::{
    data_sources::datasource::DataSource,
    models::{
        setups::setup_finder::SetupFinder, timeseries::TimeSeries, trade::Trade,
        traits::trading_strategy::TradingStrategy,
    },
};

pub struct SetupFinderBuilder {
    strategy: Option<Box<dyn TradingStrategy>>,
    ts: Option<Addr<TimeSeries>>,
    source: Option<DataSource>,
    notifications_enabled: bool,
    live_trading_enabled: bool,
    only_trigger_once: bool,
    spawned_trades: Vec<Addr<Trade>>,
}

impl SetupFinderBuilder {
    pub fn new() -> Self {
        SetupFinderBuilder {
            strategy: None,
            ts: None,
            source: None,
            notifications_enabled: false,
            live_trading_enabled: false,
            only_trigger_once: false,
            spawned_trades: vec![],
        }
    }

    pub fn strategy(mut self, strategy: Box<dyn TradingStrategy>) -> Self {
        self.strategy = Some(strategy);
        self
    }

    pub fn source(mut self, source: DataSource) -> Self {
        self.source = Some(source);
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

    pub fn only_trigger_once(mut self, enabled: bool) -> Self {
        self.only_trigger_once = enabled;
        self
    }

    #[allow(dead_code)]
    pub fn spawned_trades(mut self, trades: &[Addr<Trade>]) -> Self {
        self.spawned_trades = trades.to_vec();
        self
    }

    pub fn build(self) -> Result<SetupFinder> {
        let strategy = self
            .strategy
            .context("Strategy is required to build SetupFinder")?;
        let ts = self
            .ts
            .context("TimeSeries address is required to build SetupFinder")?;
        let notifications_enabled = self.notifications_enabled;
        let live_trading_enabled = self.live_trading_enabled;
        let only_trigger_once = self.only_trigger_once;
        let spawned_trades = self.spawned_trades;
        let source = self
            .source
            .context("Source is required to build SetupFinder")?;

        Ok(SetupFinder::new(
            strategy,
            ts,
            notifications_enabled,
            live_trading_enabled,
            only_trigger_once,
            &spawned_trades,
            source,
        )?)
    }
}
