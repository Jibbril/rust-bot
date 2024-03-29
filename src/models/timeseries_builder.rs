use super::{
    candle::Candle, interval::Interval, net_version::NetVersion, setups::setup_finder::SetupFinder,
    timeseries::TimeSeries,
};
use crate::indicators::indicator_type::IndicatorType;
use actix::Addr;
use indexmap::IndexSet;

#[derive(Debug, Clone)]
pub struct TimeSeriesBuilder {
    symbol: Option<String>,
    interval: Option<Interval>,
    max_length: usize,
    candles: Vec<Candle>,
    indicators: IndexSet<IndicatorType>,
    observers: Vec<Addr<SetupFinder>>,
    net: NetVersion,
}

#[allow(dead_code)]
impl TimeSeriesBuilder {
    pub fn new() -> Self {
        TimeSeriesBuilder {
            symbol: None,
            interval: None,
            max_length: 800,
            candles: vec![],
            indicators: IndexSet::new(),
            observers: vec![],
            net: NetVersion::Mainnet,
        }
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn interval(mut self, interval: Interval) -> Self {
        self.interval = Some(interval);
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn candles(mut self, candles: Vec<Candle>) -> Self {
        self.candles = candles;
        self
    }

    pub fn add_indicator(mut self, indicator: IndicatorType) -> Self {
        self.indicators.insert(indicator);
        self
    }

    pub fn add_observer(mut self, observer: Addr<SetupFinder>) -> Self {
        self.observers.push(observer);
        self
    }

    pub fn net(mut self, net: NetVersion) -> Self {
        self.net = net;
        self
    }

    pub fn build(self) -> TimeSeries {
        TimeSeries {
            symbol: self.symbol.expect("Symbol is required"),
            interval: self.interval.expect("Interval is required"),
            max_length: self.max_length,
            candles: self.candles,
            indicators: self.indicators,
            observers: self.observers,
            net: self.net,
        }
    }
}
