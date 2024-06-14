use crate::{
    indicators::indicator_type::IndicatorType,
    models::{
        candle::Candle, setups::setup::Setup, strategy_orientation::StrategyOrientation,
        traits::requires_indicators::RequiresIndicators,
    },
    resolution_strategies::{
        dynamic_pivot::DynamicPivotResolution, fixed_values::FixedValuesResolution,
        is_resolution_strategy::IsResolutionStrategy,
        pmarp_or_bbwp_vs_percentage::PmarpOrBbwpVsPercentageResolution,
        pmarp_vs_percentage::PmarpVsPercentageResolution,
        instant_resolution::InstantResolution,
    },
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStrategy {
    DynamicPivot(DynamicPivotResolution),
    FixedValues(FixedValuesResolution),
    PmarpVsPercentage(PmarpVsPercentageResolution),
    PmarpOrBbwpVsPercentage(PmarpOrBbwpVsPercentageResolution),
    Instant(InstantResolution),
}

impl IsResolutionStrategy for ResolutionStrategy {
    fn n_candles_stop_loss(&self) -> usize {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.n_candles_stop_loss(),
            ResolutionStrategy::FixedValues(fv) => fv.n_candles_stop_loss(),
            ResolutionStrategy::PmarpVsPercentage(pvp) => pvp.n_candles_stop_loss(),
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pbvp) => pbvp.n_candles_stop_loss(),
            ResolutionStrategy::Instant(i) => i.n_candles_stop_loss(),
        }
    }

    fn n_candles_take_profit(&self) -> usize {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.n_candles_take_profit(),
            ResolutionStrategy::FixedValues(fv) => fv.n_candles_take_profit(),
            ResolutionStrategy::PmarpVsPercentage(pvp) => pvp.n_candles_take_profit(),
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pvp) => pvp.n_candles_take_profit(),
            ResolutionStrategy::Instant(i) => i.n_candles_take_profit(),
        }
    }

    fn stop_loss_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool> {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.stop_loss_reached(orientation, candles),
            ResolutionStrategy::FixedValues(fv) => fv.stop_loss_reached(orientation, candles),
            ResolutionStrategy::PmarpVsPercentage(pvp) => {
                pvp.stop_loss_reached(orientation, candles)
            }
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pvp) => {
                pvp.stop_loss_reached(orientation, candles)
            }
            ResolutionStrategy::Instant(i) => i.stop_loss_reached(orientation, candles),
        }
    }

    fn take_profit_reached(
        &self,
        orientation: &StrategyOrientation,
        candles: &[Candle],
    ) -> Result<bool> {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.take_profit_reached(orientation, candles),
            ResolutionStrategy::FixedValues(fv) => fv.take_profit_reached(orientation, candles),
            ResolutionStrategy::PmarpVsPercentage(pvp) => {
                pvp.take_profit_reached(orientation, candles)
            }
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pvp) => {
                pvp.take_profit_reached(orientation, candles)
            }
            ResolutionStrategy::Instant(i) => i.take_profit_reached(orientation, candles),
        }
    }

    fn set_initial_values(&mut self, setup: &Setup) -> Result<()> {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.set_initial_values(setup),
            ResolutionStrategy::FixedValues(fv) => fv.set_initial_values(setup),
            ResolutionStrategy::PmarpVsPercentage(pvp) => pvp.set_initial_values(setup),
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pvp) => pvp.set_initial_values(setup),
            ResolutionStrategy::Instant(i) => i.set_initial_values(setup),
        }
    }
}

impl RequiresIndicators for ResolutionStrategy {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        match self {
            ResolutionStrategy::DynamicPivot(dp) => dp.required_indicators(),
            ResolutionStrategy::FixedValues(fv) => fv.required_indicators(),
            ResolutionStrategy::PmarpVsPercentage(pvp) => pvp.required_indicators(),
            ResolutionStrategy::PmarpOrBbwpVsPercentage(pvp) => pvp.required_indicators(),
            ResolutionStrategy::Instant(i) => i.required_indicators(),
        }
    }
}

impl Display for ResolutionStrategy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DynamicPivot(dp) => write!(f, "DynamicPivot({})", dp.len),
            Self::FixedValues(fv) => write!(f, "FixedValues({},{})", fv.high, fv.low),
            Self::PmarpVsPercentage(pvp) => {
                let init_value = match pvp.initial_value {
                    Some(v) => v,
                    None => -1.0,
                };
                write!(
                    f,
                    "PmarpVsPercentage({},{})",
                    init_value, pvp.pmarp_threshhold
                )
            }
            Self::PmarpOrBbwpVsPercentage(pvp) => {
                let init_value = match pvp.initial_value {
                    Some(v) => v,
                    None => -1.0,
                };
                write!(
                    f,
                    "PmarpOrBbwpVsPercentage({},{},{})",
                    init_value, pvp.pmarp_threshold, pvp.bbwp_threshold
                )
            }
            Self::Instant(_) => write!(f, "Instant"),
        }
    }
}
