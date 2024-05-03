use crate::
    models::{
        candle::Candle, interval::Interval, setups::csv_setup_row::CsvSetupRow,
        strategy_orientation::StrategyOrientation,
    }
;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup {
    pub candle: Candle,
    pub orientation: StrategyOrientation,
    pub symbol: String,
    pub interval: Interval,
}

impl Setup {
    #[allow(dead_code)] // TODO: Remove once used
    pub fn dummy() -> Setup {
        let candle = Candle::dummy_data(1, "", 100.0).pop().unwrap();
        Setup {
            symbol: "DUMMY".to_string(),
            candle,
            interval: Interval::Day1,
            orientation: StrategyOrientation::Long,
        }
    }

    #[allow(dead_code)] // TODO: Remove once used
    pub fn to_csv_row(&self) -> CsvSetupRow {
        CsvSetupRow {
            symbol: self.symbol.clone(),
            timestamp: self.candle.timestamp,
            interval: self.interval.clone(),
            orientation: self.orientation,
            open: self.candle.open,
            close: self.candle.close,
            high: self.candle.high,
            low: self.candle.low,
            volume: self.candle.volume,
        }
    }
}
