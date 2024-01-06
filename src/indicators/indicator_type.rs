use super::{
    atr::ATR,
    bbw::BBW,
    bbwp::BBWP,
    bollinger_bands::BollingerBands,
    dynamic_pivots::DynamicPivots,
    ema::EMA,
    indicator_args::IndicatorArgs,
    populates_candles::{PopulatesCandles, PopulatesCandlesWithSelf},
    rsi::RSI,
    sma::SMA, pmar::PMAR, pmarp::PMARP,
};
use crate::models::timeseries::TimeSeries;
use anyhow::Result;
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize)]
pub enum IndicatorType {
    SMA(usize),
    EMA(usize),
    RSI(usize),
    ATR(usize),
    BollingerBands(usize),
    BBW(usize),
    BBWP(usize, usize), // length, lookback
    DynamicPivot(usize),
    PMAR(usize),
    PMARP(usize,usize), // length, lookback
}

impl PopulatesCandlesWithSelf for IndicatorType {
    fn populate_candles(&self, ts: &mut TimeSeries) -> Result<()> {
        match self {
            IndicatorType::ATR(n) => {
                let args = IndicatorArgs::LengthArg(*n);
                ATR::populate_candles_args(ts, args)
            }
            IndicatorType::BBW(len) => {
                let args = IndicatorArgs::BollingerBandArgs(*len, 2.0);
                BBW::populate_candles_args(ts, args)
            }
            IndicatorType::BBWP(len, lookback) => {
                let args = IndicatorArgs::BBWPArgs(*len, *lookback, 5);
                BBWP::populate_candles_args(ts, args)
            }
            IndicatorType::BollingerBands(len) => {
                let args = IndicatorArgs::BollingerBandArgs(*len, 2.0);
                BollingerBands::populate_candles_args(ts, args)
            }
            IndicatorType::DynamicPivot(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                DynamicPivots::populate_candles_args(ts, args)
            }
            IndicatorType::EMA(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                EMA::populate_candles_args(ts, args)
            }
            IndicatorType::RSI(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                RSI::populate_candles_args(ts, args)
            }
            IndicatorType::SMA(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                SMA::populate_candles_args(ts, args)
            }
            IndicatorType::PMAR(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                PMAR::populate_candles_args(ts, args)
            },
            IndicatorType::PMARP(len,lookback) => {
                let args = IndicatorArgs::LengthLookbackArgs(*len, *lookback);
                PMARP::populate_candles_args(ts, args)
            }
        }
    }

    fn populate_last_candle(&self, ts: &mut TimeSeries) -> Result<()> {
        match self {
            IndicatorType::ATR(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                ATR::populate_last_candle_args(ts, args)
            }
            IndicatorType::BBW(len) => {
                let args = IndicatorArgs::BollingerBandArgs(*len, 2.0);
                BBW::populate_last_candle_args(ts, args)
            }
            IndicatorType::BBWP(len, lookback) => {
                let args = IndicatorArgs::BBWPArgs(*len, *lookback, 5);
                BBWP::populate_last_candle_args(ts, args)
            }
            IndicatorType::BollingerBands(len) => {
                let args = IndicatorArgs::BollingerBandArgs(*len, 2.0);
                BollingerBands::populate_last_candle_args(ts, args)
            }
            IndicatorType::DynamicPivot(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                DynamicPivots::populate_last_candle_args(ts, args)
            }
            IndicatorType::EMA(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                EMA::populate_last_candle_args(ts, args)
            }
            IndicatorType::RSI(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                RSI::populate_last_candle_args(ts, args)
            }
            IndicatorType::SMA(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                SMA::populate_last_candle_args(ts, args)
            }
            IndicatorType::PMAR(len) => {
                let args = IndicatorArgs::LengthArg(*len);
                PMAR::populate_last_candle_args(ts, args)
            },
            IndicatorType::PMARP(len, lookback) => {
                let args = IndicatorArgs::LengthLookbackArgs(*len, *lookback);
                PMARP::populate_last_candle_args(ts, args)
            },
        }
    }
}
