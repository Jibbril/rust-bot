mod test_result;

use crate::{
    models::candle::Candle,
    trading_strategies::{setup::Setup, strategy_orientation::StrategyOrientation},
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use test_result::TestResult;

pub fn test_setups(setups: &[Setup], candles: &Vec<Candle>) -> TestResult {
    let index_pairs = get_index_pairs(setups, candles);
    let data = gather_results_data(setups, candles, &index_pairs);
    calculate_test_result(&data)
}

fn get_index_pairs(setups: &[Setup], candles: &[Candle]) -> Vec<(usize, usize)> {
    let mut setup_index_map: HashMap<&DateTime<Utc>, usize> = HashMap::new();

    for (i, setup) in setups.iter().enumerate() {
        let key = &setup.candle.timestamp;
        setup_index_map.insert(key, i);
    }

    let mut index_pairs: Vec<(usize, usize)> = Vec::new();

    for (i, candle) in candles.iter().enumerate() {
        if setup_index_map.contains_key(&candle.timestamp) {
            let map_i = setup_index_map.get(&candle.timestamp).unwrap();

            index_pairs.push((*map_i, i))
        }
    }

    index_pairs
}

fn gather_results_data(
    setups: &[Setup],
    candles: &[Candle],
    index_pairs: &[(usize, usize)],
) -> Vec<(f64, usize, StrategyOrientation)> {
    index_pairs
        .iter()
        .filter_map(|&(setup_i, candle_i)| {
            let setup = &setups[setup_i];
            let close = setup.candle.close;
            let take_profit = setup.take_profit;
            let stop_loss = setup.stop_loss;

            let mut outcome = 0.0;
            let mut i = candle_i + 1;

            while let Some(candle) = candles.get(i) {
                let (is_win, is_loss) = match setup.orientation {
                    StrategyOrientation::Long => {
                        (candle.high >= take_profit, candle.low <= stop_loss)
                    }
                    StrategyOrientation::Short => {
                        (candle.low <= take_profit, candle.high >= stop_loss)
                    }
                };

                if is_win {
                    outcome = match setup.orientation {
                        StrategyOrientation::Long => (take_profit - close) / close,
                        StrategyOrientation::Short => (close - take_profit) / close,
                    };
                    break;
                } else if is_loss {
                    outcome = match setup.orientation {
                        StrategyOrientation::Long => (stop_loss - close) / close,
                        StrategyOrientation::Short => (close - stop_loss) / close,
                    };
                    break;
                } else {
                    i += 1;
                }
            }

            if i >= candles.len() {
                None
            } else {
                let bars = i - candle_i;
                Some((outcome, bars, setup.orientation))
            }
        })
        .collect()
}

fn calculate_test_result(data: &[(f64, usize, StrategyOrientation)]) -> TestResult {
    let mut accuracy = 0;
    let mut wins = 0;
    let mut losses = 0;
    let mut avg_win = 0.0;
    let mut avg_loss = 0.0;
    let mut avg_win_bars = 0.0;
    let mut avg_loss_bars = 0.0;

    for (outcome, bars, _) in data.iter() {
        if *outcome >= 0.0 {
            accuracy += 1;
            avg_win += *outcome;
            avg_win_bars += *bars as f64;
            wins += 1;
        } else {
            avg_loss += *outcome;
            avg_loss_bars += *bars as f64;
            losses += 1;
        }
    }

    let wins_length = if wins > 0 { wins as f64 } else { 1.0 };
    let losses_length = if losses > 0 { losses as f64 } else { 1.0 };
    let accuracy = if data.len() > 0 {
        (accuracy as f64) / data.len() as f64
    } else {
        0.0
    };
    let avg_win = avg_win / wins_length;
    let avg_loss = avg_loss / losses_length;
    let avg_win_bars = avg_win_bars / wins_length;
    let avg_loss_bars = avg_loss_bars / losses_length;

    TestResult {
        accuracy,
        n: data.len(),
        avg_win,
        avg_loss,
        avg_win_bars,
        avg_loss_bars,
    }
}

#[cfg(test)]
mod tests {
    use super::test_setups;
    use crate::{
        indicators::{rsi::RSI, PopulatesCandles},
        models::{candle::Candle, interval::Interval, timeseries::TimeSeries},
        resolution_strategies::{atr_resolution::AtrResolution, ResolutionStrategy},
        trading_strategies::{
            rsi_basic::RsiBasic,
            setup::{FindsSetups, Setup},
            strategy::Strategy,
            strategy_orientation::StrategyOrientation,
        },
    };
    use chrono::{Duration, Utc};
    use std::collections::HashMap;

    #[test]
    fn test_empty_arrays() {
        let candles = Vec::new();
        let setups = Vec::new();

        let results = test_setups(&setups, &candles);

        assert!(results.n == 0);
        assert!(results.avg_win_bars == 0.0);
        assert!(results.avg_win == 0.0);
        assert!(results.accuracy == 0.0);
        assert!(results.avg_loss == 0.0);
        assert!(results.avg_loss_bars == 0.0);
    }

    #[test]
    fn test_setup_result() {
        // Create TimeSeries
        let mut candles = Candle::dummy_data(7, "positive", 100.0);
        candles.append(&mut Candle::dummy_data(7, "negative", 170.0));
        candles.append(&mut Candle::dummy_data(10, "positive", 100.0));
        candles.append(&mut Candle::dummy_data(10, "negative", 200.0));

        let mut ts = TimeSeries {
            candles,
            ticker: "TEST".to_string(),
            interval: Interval::Daily,
        };

        // Populate rsi indicator
        let _ = RSI::populate_candles(&mut ts.candles, 14);

        // Create RSIBasic strategy
        let strat = Strategy::RsiBasic(RsiBasic::new_default());

        // Create setups from strategy and candles
        let setups = strat.find_setups(&ts);

        // Test setups
        assert!(setups.is_ok());
        let results = test_setups(&setups.unwrap(), &ts.candles);

        // Ensure values are computed correctly
        assert!(results.n == 1);
        assert!(results.avg_win_bars == 2.0);
        assert!(results.avg_win - 0.078947368 < 0.01);
        assert!(results.accuracy == 1.0);
        assert!(results.avg_loss == 0.0);
        assert!(results.avg_loss_bars == 0.0);
    }

    #[test]
    fn test_multiple_setups() {
        let long_setup = gen_candle(100.0, 1);
        let short_setup = gen_candle(150.0, 6);

        let fail_long = gen_candle(100.0, 10);
        let fail_short = gen_candle(70.0, 13);

        let candles = vec![
            gen_candle(90.0, 0),
            long_setup.clone(),
            gen_candle(130.0, 2),
            gen_candle(105.0, 3),
            gen_candle(135.0, 4),
            gen_candle(150.0, 5),
            short_setup.clone(),
            gen_candle(125.0, 7),
            gen_candle(105.0, 8),
            gen_candle(93.0, 9),
            fail_long.clone(),
            gen_candle(93.0, 11),
            gen_candle(80.0, 12),
            fail_short.clone(),
            gen_candle(85.0, 14),
            gen_candle(91.0, 15),
        ];

        let resolution_strategy = ResolutionStrategy::ATR(AtrResolution::new(14, 1.0, 1.5));

        let setups = vec![
            Setup {
                candle: long_setup,
                ticker: "TEST".to_string(),
                take_profit: 150.0,
                stop_loss: 95.0,
                interval: Interval::Daily,
                orientation: StrategyOrientation::Long,
                resolution_strategy: resolution_strategy.clone(),
            },
            Setup {
                candle: short_setup,
                ticker: "TEST".to_string(),
                take_profit: 105.0,
                stop_loss: 155.0,
                interval: Interval::Daily,
                orientation: StrategyOrientation::Short,
                resolution_strategy: resolution_strategy.clone(),
            },
            Setup {
                candle: fail_long,
                ticker: "TEST".to_string(),
                take_profit: 130.0,
                stop_loss: 80.0,
                interval: Interval::Daily,
                orientation: StrategyOrientation::Long,
                resolution_strategy: resolution_strategy.clone(),
            },
            Setup {
                candle: fail_short,
                ticker: "TEST".to_string(),
                take_profit: 50.0,
                stop_loss: 91.0,
                interval: Interval::Daily,
                orientation: StrategyOrientation::Short,
                resolution_strategy: resolution_strategy,
            },
        ];

        let results = test_setups(&setups, &candles);

        assert!(results.n == 4);
        assert!(results.avg_win_bars == 3.0);
        assert!(results.avg_win == 0.4);
        assert!(results.accuracy == 0.5);
        assert!(results.avg_loss == -0.25);
        assert!(results.avg_loss_bars == 2.0);
    }

    fn gen_candle(val: f64, increment: i64) -> Candle {
        Candle {
            open: val,
            low: val,
            high: val,
            close: val,
            timestamp: Utc::now() + Duration::days(increment),
            volume: 1000.0,
            indicators: HashMap::new(),
        }
    }
}
