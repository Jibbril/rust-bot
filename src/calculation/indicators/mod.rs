mod calculation_mode;
mod sma;
mod rsi;

use sma::SMA;
use rsi::RSI;

use crate::utils::timeseries::Candle;

pub trait PopulatesCandles {
    fn populate_candles(candles: &mut Vec<Candle>);
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
pub enum IndicatorType {
    SMA(usize),
    RSI(usize),
}

#[derive(Debug,Clone)]
pub enum Indicator {
    SMA(SMA),
    RSI(RSI),
}