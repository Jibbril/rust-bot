use super::{
    atr::ATR, bbw::BBW, bbwp::BBWP, bollinger_bands::BollingerBands, dynamic_pivots::DynamicPivot,
    ema::EMA, rsi::RSI, sma::SMA,
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
