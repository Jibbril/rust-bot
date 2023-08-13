use crate::models::{candle::Candle, generic_result::GenericResult};

use super::{bollinger_bands::BollingerBands, PopulatesCandles, IndicatorType, Indicator};


/// Bollinger Band Width
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct BBW {
    #[allow(dead_code)] // TODO: Remove once used
    pub bb: BollingerBands,
    pub value: f64
}

impl PopulatesCandles for BBW {
    fn populate_candles(candles: &mut Vec<Candle>, length: usize) -> GenericResult<()> {
        let mut bbw: Option<BBW> = None;
        let new_bbws: Vec<Option<BBW>> = (0..candles.len())
            .map(|i|  {
                bbw = Self::calculate_rolling(length, i, candles,&bbw);
                bbw
            })
            .collect();

        let indicator_type = IndicatorType::BBW(length);

        for (i, candle) in candles.iter_mut().enumerate() {
            let new_bb = Indicator::BBW(new_bbws[i]);

            candle.indicators.insert(indicator_type, new_bb);
        }

        Ok(())
    }
}

impl BBW {
    pub fn calculate(
        length: usize,
        i: usize,
        candles: &[Candle],
    ) -> Option<BBW> {
        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            None
        } else {
            let bb = BollingerBands::calculate(length, i, candles)?;
            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb)
            })
        }
    }

    pub fn calculate_rolling(
        length: usize,
        i: usize,
        candles: &[Candle],
        prev_bbw: &Option<BBW>
    ) -> Option<BBW> {
        if !BollingerBands::calculation_ok(i, length, candles.len()) {
            return None
        } else if let Some(prev_bbw) = prev_bbw {
            let prev_bb = Some(prev_bbw.bb);
            let bb = BollingerBands::calculate_rolling(length, i, candles, &prev_bb)?;
            
            Some(BBW {
                bb,
                value: Self::calculate_bbw(&bb)
            })
        } else {
            Self::calculate(length, i, candles)
        }
    }

    fn calculate_bbw(bb: &BollingerBands) -> f64 {
        (bb.upper - bb.lower) / bb.sma.value
    }
}