use anyhow::{Result, anyhow, Context};
use serde::{Serialize, Deserialize};
use crate::{models::{strategy_orientation::StrategyOrientation, candle::Candle, traits::requires_indicators::RequiresIndicators, setups::setup::Setup}, indicators::indicator_type::IndicatorType};
use super::is_resolution_strategy::IsResolutionStrategy;

/// # PmarpVsPercentageResolution
///
/// Resolution strategy which utilizes pmarp values for take-profit determination
/// and a percentage drawdown for stop-loss.
///
/// ## Trading orientations
/// - Long
///
/// ## Suggested values
/// - pmarp_threshold = 68
/// - % drawdown = 4.5%
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PmarpVsPercentageResolution {
    pub pmarp_threshhold: f64,
    pub initial_value: Option<f64>,
    pub drawdown_threshold: f64,
    pub pmarp_len: usize,
    pub pmarp_lookback: usize,
}

const ERR_MESSAGE: &str = "pmarp vs % resolution does not support short orientation.";

impl IsResolutionStrategy for PmarpVsPercentageResolution {
    fn n_candles_stop_loss(&self) -> usize {
        1
    }

    fn n_candles_take_profit(&self) -> usize {
        1
    }

    fn stop_loss_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        let len = candles.len();

        if len == 0 { 
            return Err(anyhow!("No candle passed for pmarp vs % resolution."))
        }

        match orientation {
            StrategyOrientation::Long => {
                let init_value = self.initial_value.context("Expected initial value")?;
                Ok(
                    (1.0 - &candles[len-1].close / init_value) * 100.0 > self.drawdown_threshold
                )
            },
            StrategyOrientation::Short => Err(anyhow!(ERR_MESSAGE))
        }
    }

    fn take_profit_reached(&self, orientation: &StrategyOrientation, candles: &[Candle]) -> Result<bool> {
        let len = candles.len();
        if len == 0 { 
            return Err(anyhow!("No candle passed for pmarp vs % resolution."))
        }

        let ind_type = IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback);
        let pmarp = candles[len-1].indicators.get(&ind_type)
            .context("Unable to get pmarp for pmarp vs % resolution.")?
            .as_pmarp()
            .context("Unable to convert indicator to pmarp in pmarp vs % resolution")?;

        match orientation {
            StrategyOrientation::Long => Ok(pmarp.value > self.pmarp_threshhold),
            StrategyOrientation::Short => Err(anyhow!(ERR_MESSAGE))
        }
    }

    fn set_initial_values(&mut self, setup: &Setup) -> Result<()> {
        self.initial_value = Some(setup.candle.close);

        Ok(())
    }
}

impl RequiresIndicators for PmarpVsPercentageResolution {
    fn required_indicators(&self) -> Vec<IndicatorType> {
        vec![IndicatorType::PMARP(self.pmarp_len, self.pmarp_lookback)]
    }
}
