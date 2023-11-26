use crate::models::timeseries::TimeSeries;
use super::{
    atr::ATR, bbw::BBW, bbwp::BBWP, bollinger_bands::BollingerBands, dynamic_pivots::DynamicPivot,
    ema::EMA, indicator_type::IndicatorType, rsi::RSI, sma::SMA,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Indicator {
    MA(MovingAverage),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
    BollingerBands(Option<BollingerBands>),
    BBW(Option<BBW>),
    BBWP(Option<BBWP>),
    DynamicPivot(Option<DynamicPivot>),
}

#[derive(Debug, Clone)]
pub enum MovingAverage {
    Simple(Option<SMA>),
    Exponential(Option<EMA>),
}

impl Indicator {
    /// Returns the nth last indicator of the given type for the given TimeSeries.
    ///
    /// # Arguments
    /// * `ts` - The TimeSeries to get the nth last indicator from.
    /// * `indicator_type` - The type of indicator to get the nth last indicator of.
    /// * `i` - The index of the indicator to get. 0 Is last, 1 is second last, etc.
    pub fn get_nth_last(
        ts: &TimeSeries,
        indicator_type: &IndicatorType,
        i: usize,
    ) -> Option<Indicator> {
        let candles_len = ts.candles.len();

        if candles_len < i + 1 {
            return None;
        }

        let prev = ts
            .candles
            .get(candles_len - 1 - i)
            .and_then(|candle| candle.indicators.get(&indicator_type))
            .and_then(|indicator| Some(indicator.clone()));

        prev.clone()
    }

    /// Returns the second last indicator of the given type for the given TimeSeries.
    ///
    /// # Arguments
    /// * `ts` - The TimeSeries to get the second last indicator from.
    /// * `indicator_type` - The type of indicator to get the second last indicator of.
    pub fn get_second_last(ts: &TimeSeries, indicator_type: &IndicatorType) -> Option<Indicator> {
        Self::get_nth_last(ts, indicator_type, 2)
    }

    pub fn as_sma(&self) -> Option<SMA> {
        let ma = match self {
            Indicator::MA(ma) => ma,
            _ => return None,
        };

        match ma {
            MovingAverage::Simple(s) => s.clone(),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn as_ema(&self) -> Option<EMA> {
        let ma = match self {
            Indicator::MA(ma) => ma,
            _ => return None,
        };

        match ma {
            MovingAverage::Exponential(e) => e.clone(),
            _ => None,
        }
    }

    pub fn as_rsi(&self) -> Option<RSI> {
        if let Indicator::RSI(rsi) = self {
            rsi.clone()
        } else {
            None
        }
    }

    pub fn as_atr(&self) -> Option<ATR> {
        if let Indicator::ATR(atr) = self {
            atr.clone()
        } else {
            None
        }
    }

    pub fn as_dynamic_pivots(&self) -> Option<DynamicPivot> {
        if let Indicator::DynamicPivot(pivots) = self {
            pivots.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bollinger_bands(&self) -> Option<BollingerBands> {
        if let Indicator::BollingerBands(bb) = self {
            bb.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bbw(&self) -> Option<BBW> {
        if let Indicator::BBW(bbw) = self {
            bbw.clone()
        } else {
            None
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_bbwp(&self) -> Option<BBWP> {
        if let Indicator::BBWP(bbwp) = self {
            bbwp.clone()
        } else {
            None
        }
    }
}
