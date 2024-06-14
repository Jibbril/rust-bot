use crate::{
    indicators::{
        atr::ATR,
        bbw::BBW,
        bbwp::BBWP,
        bollinger_bands::BollingerBands,
        dynamic_pivots::DynamicPivots,
        ema::EMA,
        indicator_args::IndicatorArgs,
        pmar::PMAR,
        pmarp::PMARP,
        populates_candles::{PopulatesCandles, PopulatesCandlesWithSelf},
        rsi::RSI,
        sma::SMA,
        stochastic::Stochastic,
    },
    models::{ma_type::MAType, timeseries::TimeSeries, traits::has_min_length::HasMinLength},
};
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
    PMAR(usize, MAType),
    PMARP(usize, usize, MAType),     // length, lookback
    Stochastic(usize, usize, usize), // K-lenght, K-smoothing, D-smoothing
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
            IndicatorType::PMAR(len, ma_type) => {
                let args = IndicatorArgs::PMARArgs(*len, *ma_type);
                PMAR::populate_candles_args(ts, args)
            }
            IndicatorType::PMARP(len, lookback, ma_type) => {
                let args = IndicatorArgs::PMARPArgs(*len, *lookback, *ma_type);
                PMARP::populate_candles_args(ts, args)
            }
            IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing) => {
                let args = IndicatorArgs::StochasticArgs(*k_len, *k_smoothing, *d_smoothing);
                Stochastic::populate_candles_args(ts, args)
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
            IndicatorType::PMAR(len, ma_type) => {
                let args = IndicatorArgs::PMARArgs(*len, *ma_type);
                PMAR::populate_last_candle_args(ts, args)
            }
            IndicatorType::PMARP(len, lookback, ma_type) => {
                let args = IndicatorArgs::PMARPArgs(*len, *lookback, *ma_type);
                PMARP::populate_last_candle_args(ts, args)
            }
            IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing) => {
                let args = IndicatorArgs::StochasticArgs(*k_len, *k_smoothing, *d_smoothing);
                Stochastic::populate_last_candle_args(ts, args)
            }
        }
    }
}

impl HasMinLength for IndicatorType {
    fn min_length(&self) -> usize {
        match self {
            IndicatorType::SMA(n) => n + 1,
            IndicatorType::EMA(n) => n + 1,
            IndicatorType::RSI(n) => n + 1,
            IndicatorType::ATR(n) => n + 1,
            IndicatorType::BollingerBands(n) => n + 1,
            IndicatorType::BBW(n) => n + 1,
            IndicatorType::BBWP(_, n) => n + 1,
            IndicatorType::DynamicPivot(n) => n + 1,
            IndicatorType::PMAR(n, _) => n + 1,
            IndicatorType::PMARP(_, n, _) => n + 1,
            IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing) => {
                k_len + k_smoothing + d_smoothing
            }
        }
    }
}
