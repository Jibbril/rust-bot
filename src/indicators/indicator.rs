use crate::{
    indicators::{
        atr::ATR, bbw::BBW, bbwp::BBWP, bollinger_bands::BollingerBands,
        dynamic_pivots::DynamicPivots, ema::EMA, indicator_type::IndicatorType, pmar::PMAR,
        pmarp::PMARP, rsi::RSI, sma::SMA,
    },
    models::timeseries::TimeSeries,
};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Indicator {
    SMA(Option<SMA>),
    EMA(Option<EMA>),
    RSI(Option<RSI>),
    ATR(Option<ATR>),
    BollingerBands(Option<BollingerBands>),
    BBW(Option<BBW>),
    BBWP(Option<BBWP>),
    DynamicPivot(Option<DynamicPivots>),
    PMAR(Option<PMAR>),
    PMARP(Option<PMARP>),
}

impl Indicator {
    /// Returns the nth last indicator of the given type for the given TimeSeries.
    ///
    /// # Arguments
    /// * `ts` - The TimeSeries to get the nth last indicator from.
    /// * `indicator_type` - The type of indicator to get the nth last indicator of.
    /// * `i` - The index of the indicator to get. 1 Is last, 2 is second last, etc.
    pub fn get_nth_last(
        ts: &TimeSeries,
        indicator_type: &IndicatorType,
        i: usize,
    ) -> Option<Indicator> {
        let candles_len = ts.candles.len();

        if candles_len < i {
            return None;
        }

        let prev = ts
            .candles
            .get(candles_len - i)
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

    #[allow(dead_code)] // TODO: Remove once used
    pub fn as_sma(&self) -> Option<SMA> {
        if let Indicator::SMA(sma) = self {
            sma.clone()
        } else {
            None
        }
    }

    pub fn as_ema(&self) -> Option<EMA> {
        if let Indicator::EMA(ema) = self {
            ema.clone()
        } else {
            None
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

    pub fn as_dynamic_pivots(&self) -> Option<DynamicPivots> {
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

    pub fn as_bbw(&self) -> Option<BBW> {
        if let Indicator::BBW(bbw) = self {
            bbw.clone()
        } else {
            None
        }
    }

    pub fn as_bbwp(&self) -> Option<BBWP> {
        if let Indicator::BBWP(bbwp) = self {
            bbwp.clone()
        } else {
            None
        }
    }

    pub fn as_pmar(&self) -> Option<PMAR> {
        if let Indicator::PMAR(pmar) = self {
            pmar.clone()
        } else {
            None
        }
    }

    pub fn as_pmarp(&self) -> Option<PMARP> {
        if let Indicator::PMARP(pmarp) = self {
            pmarp.clone()
        } else {
            None
        }
    }
}
