use crate::{
    models::{
        candle::Candle, interval::Interval, setups::csv_setup_row::CsvSetupRow,
        strategy_orientation::StrategyOrientation,
    },
    resolution_strategies::{
        dynamic_pivot::DynamicPivotResolution, resolution_strategy::ResolutionStrategy,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub candle: Candle,
    pub orientation: StrategyOrientation,
    pub symbol: String,
    pub interval: Interval,
    pub resolution_strategy: Option<ResolutionStrategy>,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        let resolution_strategy = ResolutionStrategy::DynamicPivot(DynamicPivotResolution::new());
        Setup {
            symbol: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
            resolution_strategy: Some(resolution_strategy),
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn to_csv_row(&self) -> CsvSetupRow {
        let resolution = match self.resolution_strategy {
            Some(ref strategy) => strategy.to_string(),
            None => "-".to_string(),
        };

        CsvSetupRow {
            symbol: self.symbol.clone(),
            timestamp: self.candle.timestamp,
            interval: self.interval.clone(),
            orientation: self.orientation,
            stop_loss_resolution: resolution.clone(),
            take_profit_resolution: resolution,
            open: self.candle.open,
            close: self.candle.close,
            high: self.candle.high,
            low: self.candle.low,
            volume: self.candle.volume,
        }
    }
}
