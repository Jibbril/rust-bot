use crate::{
    models::{candle::Candle, generic_result::GenericResult, timeseries::TimeSeries},
    utils::math::std,
};

use super::{sma::SMA, PopulatesCandles, indicator::Indicator, indicator_type::IndicatorType};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BollingerBands {
    pub upper: f64,
    pub lower: f64,
    pub sma: SMA,
    pub std: f64,
    pub length: usize,
}

impl PopulatesCandles for BollingerBands {
    fn populate_candles(ts: &mut TimeSeries, length: usize) -> GenericResult<()> {
        let mut bb: Option<BollingerBands> = None;
        let new_bbs: Vec<Option<BollingerBands>> = (0..ts.candles.len())
            .map(|i| {
                bb = Self::calculate_rolling(length, i, &ts.candles, &bb);
                bb
            })
            .collect();

        let indicator_type = IndicatorType::BollingerBands(length);

        for (i, candle) in ts.candles.iter_mut().enumerate() {
            let new_bb = Indicator::BollingerBands(new_bbs[i]);

            candle.indicators.insert(indicator_type, new_bb);
        }

        Ok(())
    }

    fn populate_candles_default(ts: &mut TimeSeries) -> GenericResult<()> {
        Self::populate_candles(ts, 20)
    }
}

impl BollingerBands {
    pub fn calculation_ok(i: usize, len: usize, arr_len: usize) -> bool {
        i < arr_len && len <= arr_len && i >= len - 1 && len > 0
    }

    fn typical_price(c: &Candle) -> f64 {
        (c.high + c.low + c.close) / 3.0
    }

    #[allow(dead_code)]
    pub fn calculate(length: usize, i: usize, candles: &[Candle]) -> Option<BollingerBands> {
        if !Self::calculation_ok(i, length, candles.len()) {
            None
        } else {
            let start = i + 1 - length;
            let end = i + 1;
            let segment = &candles[start..end];

            // typical price sum
            let tps: Vec<f64> = segment.iter().map(|c| Self::typical_price(c)).collect();

            let ma = tps.iter().sum::<f64>() / (length as f64);
            let std = std(&tps, ma);

            let std_n = 2.0;
            let upper = ma + std_n * std;
            let lower = ma - std_n * std;

            Some(BollingerBands {
                upper,
                lower,
                std,
                sma: SMA { length, value: ma },
                length,
            })
        }
    }

    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &[Candle],
        previous_bb: &Option<BollingerBands>,
    ) -> Option<BollingerBands> {
        if !Self::calculation_ok(i, length, candles.len()) {
            return None;
        } else if let Some(prev_bb) = previous_bb {
            let f_length = length as f64;
            let price_in = Self::typical_price(&candles[i]);
            let price_out = Self::typical_price(&candles[i - length]);
            let old_sma = prev_bb.sma.value;

            let new_sma = old_sma + (price_in - price_out) / f_length;

            let new_var = prev_bb.std.powi(2)
                + ((price_in - old_sma) * (price_in - new_sma)
                    - (price_out - old_sma) * (price_out - new_sma))
                    / f_length;

            let new_std = new_var.sqrt();

            let std_n = 2.0;
            let upper = new_sma + std_n * new_std;
            let lower = new_sma - std_n * new_std;

            Some(BollingerBands {
                upper,
                lower,
                std: new_std,
                sma: SMA {
                    length,
                    value: new_sma,
                },
                length,
            })
        } else {
            Self::calculate(length, i, candles)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BollingerBands;
    use crate::models::candle::Candle;

    #[test]
    fn calculate_bollinger_bands() {
        let candles = Candle::dummy_data(20, "positive", 100.0);

        let bb = BollingerBands::calculate(10, 19, &candles);
        assert!(bb.is_some());
        let bb = bb.unwrap();
        assert!(bb.upper - 315.5530070819 < 0.0001)
    }

    #[test]
    fn bb_not_enough_data() {
        let candles = Candle::dummy_data(2, "positive", 100.0);

        let bb = BollingerBands::calculate(20, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn bb_no_candles() {
        let candles: Vec<Candle> = Vec::new();

        let bb = BollingerBands::calculate(20, 19, &candles);

        assert!(bb.is_none());
    }

    #[test]
    fn rolling_bb() {
        let n = 40;
        let length = 20;
        let candles = Candle::dummy_data(n, "positive", 100.0);

        let mut bb: Option<BollingerBands> = None;
        let bbs: Vec<Option<BollingerBands>> = (0..candles.len())
            .map(|i| {
                bb = BollingerBands::calculate_rolling(length, i, &candles, &bb);
                bb
            })
            .collect();

        for (i, bb) in bbs.iter().enumerate() {
            if i < length - 1 {
                assert!(bb.is_none())
            } else {
                assert!(bb.is_some())
            }
        }

        assert!(bbs[n - 1].unwrap().upper - 523.3215956619 < 0.00001)
    }
}
