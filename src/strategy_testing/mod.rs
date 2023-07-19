use std::collections::HashMap;

use chrono::{DateTime, Utc};

use crate::{
    models::candle::Candle,
    trading_strategies::{setup::Setup, strategy_orientation::StrategyOrientation},
};

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
    let results = index_pairs
        .iter()
        .filter_map(|(setup_i, candle_i)| {
            let setup = &setups[*setup_i];
            let close = setup.candle.close;
            let take_profit = setup.take_profit;
            let stop_loss = setup.stop_loss;
            let orientation = setup.orientation.clone();

            let mut outcome = 0.0;
            let mut i = *candle_i;

            while let Some(candle) = candles.get(i) {
                let (is_win,is_loss) = match setup.orientation {
                    StrategyOrientation::Long => (
                        candle.high >= take_profit,
                        candle.low <= stop_loss
                    ),
                    StrategyOrientation::Short => (
                        candle.low <= take_profit,
                        candle.high >= stop_loss
                    )
                };

                if is_win {
                    outcome = match setup.orientation {
                        StrategyOrientation::Long => (take_profit - close) / close,
                        StrategyOrientation::Short => (close - take_profit) / close
                    };
                    break;
                } else if is_loss {
                    outcome = match setup.orientation {
                        StrategyOrientation::Long => (stop_loss - close) / close,
                        StrategyOrientation::Short => (close - stop_loss) / close
                    };
                    break;
                } else {
                    i += 1;
                }

            }

            if i >= candles.len() {
                return None;
            } else {
                let bars = i - candle_i;

                Some((outcome, bars, orientation))
            }
        })
        .collect();

    results
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
    let accuracy = accuracy as f64;
    let avg_win = avg_win / wins_length;
    let avg_loss = 100.0 * avg_loss / losses_length;
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
pub struct TestResult {
    accuracy: f64,
    n: usize,
    avg_win: f64,
    avg_loss: f64,
    avg_win_bars: f64,
    avg_loss_bars: f64,
}
