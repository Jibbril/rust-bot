use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle, interval::Interval,
        message_payloads::candle_added_payload::CandleAddedPayload, net_version::NetVersion,
        timeseries::TimeSeries,
    },
};
use actix::Recipient;
use indexmap::IndexSet;

#[derive(Debug, Clone)]
pub struct TimeSeriesBuilder {
    symbol: Option<String>,
    interval: Option<Interval>,
    max_length: usize,
    candles: Vec<Candle>,
    indicators: IndexSet<IndicatorType>,
    observers: Vec<Recipient<CandleAddedPayload>>,
    net: NetVersion,
    validate_candles_on_add: bool,
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
            validate_candles_on_add: true,
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

    pub fn add_observer(mut self, observer: Recipient<CandleAddedPayload>) -> Self {
        self.observers.push(observer);
        self
    }

    pub fn net(mut self, net: NetVersion) -> Self {
        self.net = net;
        self
    }

    pub fn validate_candles_on_add(mut self, b: bool) -> Self {
        self.validate_candles_on_add = b;
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
            validate_candles_on_add: self.validate_candles_on_add,
        }
    }
}
