use anyhow::{Result, Context};
use crate::{models::{traits::trading_strategy::TradingStrategy, candle::Candle, interval::Interval}, resolution_strategies::{resolution_strategy::ResolutionStrategy, is_resolution_strategy::IsResolutionStrategy}, strategy_testing::strategy_test_result_builder::StrategyTestResultBuilder};
use super::strategy_test_result::StrategyTestResult;

#[allow(dead_code)]
pub struct StrategyTester;

impl StrategyTester {
    #[allow(dead_code)]
    pub fn test_strategy(strat: &Box<dyn TradingStrategy>, candles: &[Candle]) -> Result<StrategyTestResult>{
        let orientation = strat.orientation();
        let needed_candles = strat.candles_needed_for_setup();
        let mut result_builder = StrategyTestResultBuilder::new();
        let mut next_i = 0;

        // Loop over the needed candles to determine a setup and gather results.
        for (i,window) in candles.windows(needed_candles).enumerate() {
            // Bump index up to compensate for window size
            let i = i + needed_candles;

            if i < needed_candles || i < next_i { continue }

            let sb = strat.check_last_for_setup(&window); 
            
            if sb.is_none() { continue };

            let sb = sb.context("Expected SetupBuilder.")?;
            let setup = sb.ticker("TESTING")
                .interval(&Interval::Day1)
                .build()?;

            // Initialize resolution strategy
            let mut resolution_strategy = strat.default_resolution_strategy();
            resolution_strategy.set_initial_values(&setup)?;
            let mut n_bars = 0;


            // Loop over upcoming candles to determine outcome of setup
            loop {
                n_bars += 1;

                // Max number of bars in a setup to avoid infinite loops
                if n_bars > 100 { break };

                let end = i + n_bars;
                if end > candles.len() { break };

                let tp_candles_needed = resolution_strategy.n_candles_take_profit();
                let tp_candles = &candles[end - tp_candles_needed..end];
                let take_profit_reached = resolution_strategy.take_profit_reached(&orientation, tp_candles)?;

                if take_profit_reached {
                    // println!("SETUP CANDLE: {:#?}",setup.candle);
                    // println!("TAKE-PROFIT CANDLE: {:#?}",tp_candles[tp_candles.len()-1]);
                    let increase = tp_candles[tp_candles.len()-1].close / setup.candle.close - 1.0;
                    result_builder.add_outcome(increase, n_bars);

                    break;
                }

                let sl_candles_needed = resolution_strategy.n_candles_stop_loss();
                let sl_candles = &candles[end - sl_candles_needed..end];
                let stop_loss_reached = resolution_strategy.stop_loss_reached(&orientation, sl_candles)?;

                if stop_loss_reached {
                    // println!("SETUP CANDLE: {:#?}",setup.candle);
                    // println!("STOP-LOSS CANDLE: {:#?}",sl_candles[sl_candles.len()-1]);
                    let decrease = sl_candles[sl_candles.len()-1].close / setup.candle.close - 1.0;
                    result_builder.add_outcome(decrease, n_bars);
                    break;
                }
            }

            // Avoid scenarios where the same strategy can trigger new setups 
            // while there is already a setup playing out.
            next_i = i + n_bars;
        }

        Ok(result_builder.build())
    }

    #[allow(dead_code)]
    pub fn by_strategies(_trading_strat: &Box<dyn TradingStrategy>, _resolution_strat: &ResolutionStrategy, _candles: &[Candle]) -> StrategyTestResult {
        todo!()
    }
}


// CODE BELOW IS INITIAL TESTING STRUCTURE
// pub fn test_setups(setups: &[Setup], candles: &Vec<Candle>) -> TestResult {
//     let index_pairs = get_index_pairs(setups, candles);
//     let data = gather_results_data(setups, candles, &index_pairs);
//     calculate_test_result(&data)
// }
//
// fn get_index_pairs(setups: &[Setup], candles: &[Candle]) -> Vec<(usize, usize)> {
//     let mut setup_index_map: HashMap<&DateTime<Utc>, usize> = HashMap::new();
//
//     for (i, setup) in setups.iter().enumerate() {
//         let key = &setup.candle.timestamp;
//         setup_index_map.insert(key, i);
//     }
//
//     let mut index_pairs: Vec<(usize, usize)> = Vec::new();
//
//     for (i, candle) in candles.iter().enumerate() {
//         if setup_index_map.contains_key(&candle.timestamp) {
//             let map_i = setup_index_map.get(&candle.timestamp).unwrap();
//
//             index_pairs.push((*map_i, i))
//         }
//     }
//
//     index_pairs
// }
//
// fn gather_results_data(
//     setups: &[Setup],
//     candles: &[Candle],
//     index_pairs: &[(usize, usize)],
// ) -> Vec<(f64, usize, StrategyOrientation)> {
//     index_pairs
//         .iter()
//         .filter_map(|&(setup_i, candle_i)| {
//             let setup = &setups[setup_i];
//             let close = setup.candle.close;
//             let take_profit = setup.take_profit?;
//             let stop_loss = setup.stop_loss?;
//
//             let mut outcome = 0.0;
//             let mut i = candle_i + 1;
//
//             while let Some(candle) = candles.get(i) {
//                 let (is_win, is_loss) = match setup.orientation {
//                     StrategyOrientation::Long => {
//                         // (candle.high >= take_profit, candle.low <= stop_loss);
//                         // let candles_required = setup.resolution_strategy.n_candles_for_stop_loss;
//                         // setup.resolution_strategy.stop_loss_reached(&setup.orientation, &candles[i+1-candles_required..i+1])
//                         (false,false)
//                     }
//                     // StrategyOrientation::Long => {
//                     //     (candle.high >= take_profit, candle.low <= stop_loss)
//                     // }
//                     StrategyOrientation::Short => {
//                         (candle.low <= take_profit, candle.high >= stop_loss)
//                     }
//                 };
//
//                 if is_win {
//                     outcome = match setup.orientation {
//                         StrategyOrientation::Long => (take_profit - close) / close,
//                         StrategyOrientation::Short => (close - take_profit) / close,
//                     };
//                     break;
//                 } else if is_loss {
//                     outcome = match setup.orientation {
//                         StrategyOrientation::Long => (stop_loss - close) / close,
//                         StrategyOrientation::Short => (close - stop_loss) / close,
//                     };
//                     break;
//                 } else {
//                     i += 1;
//                 }
//             }
//
//             if i >= candles.len() {
//                 None
//             } else {
//                 let bars = i - candle_i;
//                 Some((outcome, bars, setup.orientation))
//             }
//         })
//         .collect()
// }
//
// fn calculate_test_result(data: &[(f64, usize, StrategyOrientation)]) -> TestResult {
//     let mut accuracy = 0;
//     let mut wins = Vec::new();
//     let mut losses = Vec::new();
//     let mut win_bars = Vec::new();
//     let mut loss_bars = Vec::new();
//
//     for (outcome, bars, _) in data.iter() {
//         if *outcome >= 0.0 {
//             accuracy += 1;
//             wins.push(*outcome);
//             win_bars.push(*bars as f64);
//         } else {
//             losses.push(*outcome);
//             loss_bars.push(*bars as f64);
//         }
//     }
//
//     let accuracy = if !data.is_empty() {
//         (accuracy as f64) / data.len() as f64
//     } else {
//         0.0
//     };
//
//     let wins_len = f_len_or_one(&wins);
//     let losses_len = f_len_or_one(&losses);
//     let avg_win = wins.iter().sum::<f64>() / wins_len;
//     let avg_loss = losses.iter().sum::<f64>() / losses_len;
//     let avg_win_bars = win_bars.iter().sum::<f64>() / wins_len;
//     let avg_loss_bars = loss_bars.iter().sum::<f64>() / losses_len;
//     let avg_profitability = accuracy * avg_win + (1.0 - accuracy) * avg_loss;
//
//     let wins_std = std(&wins, avg_win);
//     let losses_std = std(&losses, avg_loss);
//     let win_bars_std = std(&win_bars, avg_win_bars);
//     let loss_bars_std = std(&loss_bars, avg_loss_bars);
//
//     TestResult {
//         accuracy,
//         n_setups: data.len(),
//         avg_profitability,
//         avg_win,
//         avg_loss,
//         avg_win_bars,
//         avg_loss_bars,
//         wins_std,
//         losses_std,
//         win_bars_std,
//         loss_bars_std,
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::test_setups;
//     use crate::{
//         models::{
//             candle::Candle, interval::Interval, setups::setup::Setup,
//             strategy_orientation::StrategyOrientation,
//         }, resolution_strategies::{resolution_strategy::ResolutionStrategy, dynamic_pivot::DynamicPivotResolution},
//     };
//     use chrono::{Duration, Utc};
//     use std::collections::HashMap;
//
//     #[test]
//     fn test_empty_arrays() {
//         let candles = Vec::new();
//         let setups = Vec::new();
//
//         let results = test_setups(&setups, &candles);
//
//         assert!(results.n_setups == 0);
//         assert!(results.avg_win_bars == 0.0);
//         assert!(results.avg_win == 0.0);
//         assert!(results.accuracy == 0.0);
//         assert!(results.avg_loss == 0.0);
//         assert!(results.avg_loss_bars == 0.0);
//         assert_eq!(results.wins_std, 0.0);
//         assert_eq!(results.losses_std, 0.0);
//         assert_eq!(results.win_bars_std, 0.0);
//         assert_eq!(results.loss_bars_std, 0.0);
//     }
//
//     // #[test]
//     // fn test_setup_result() {
//     //     // Create TimeSeries
//     //     // TODO: Test below fails since adding checks for timeseries
//     //     // date validity (date gets reset each time we call the dummy_data
//     //     // function. Figure out why.
//     //     let mut candles = Candle::dummy_data(7, "positive", 100.0);
//     //     candles.append(&mut Candle::dummy_data(7, "negative", 170.0));
//     //     candles.append(&mut Candle::dummy_data(10, "positive", 100.0));
//     //     candles.append(&mut Candle::dummy_data(10, "negative", 200.0));
//     //
//     //     let mut ts = TimeSeries::new("TEST".to_string(), Interval::Day1, candles);
//     //
//     //     // Harmonize dates
//     //     let timestamp = Utc::now();
//     //     let step = Duration::days(1);
//     //     ts.candles.iter_mut().for_each(|c| {
//     //         c.timestamp = timestamp;
//     //         let _ = timestamp.add(step);
//     //     });
//     //
//     //     // Populate rsi indicator
//     //     let args = IndicatorArgs::LengthArg(14);
//     //     let _ = RSI::populate_candles_args(&mut ts, args);
//     //
//     //     // Create RSIBasic strategy
//     //     let strat = RsiBasic::new_default();
//     //
//     //     // Create setups from strategy and candles
//     //     let setups = strat.find_setups(&ts);
//     //
//     //     // Test setups
//     //     assert!(setups.is_ok());
//     //     let results = test_setups(&setups.unwrap(), &ts.candles);
//     //
//     //     // Ensure values are computed correctly
//     //     assert_eq!(results.n_setups, 1);
//     //     assert_eq!(results.avg_win_bars, 1.0);
//     //     assert!(results.avg_win - 0.078947368 < 0.01);
//     //     assert_eq!(results.accuracy, 1.0);
//     //     assert_eq!(results.avg_loss, 0.0);
//     //     assert_eq!(results.avg_loss_bars, 0.0);
//     //     assert_eq!(results.wins_std, 0.0);
//     //     assert_eq!(results.losses_std, 0.0);
//     //     assert_eq!(results.win_bars_std, 0.0);
//     //     assert_eq!(results.loss_bars_std, 0.0);
//     // }
//
//     #[test]
//     fn test_multiple_setups() {
//         let long_setup = gen_candle(100.0, 1);
//         let short_setup = gen_candle(150.0, 6);
//
//         let fail_long = gen_candle(100.0, 10);
//         let fail_short = gen_candle(70.0, 13);
//
//         let candles = vec![
//             gen_candle(90.0, 0),
//             long_setup.clone(),
//             gen_candle(130.0, 2),
//             gen_candle(105.0, 3),
//             gen_candle(135.0, 4),
//             gen_candle(150.0, 5),
//             short_setup.clone(),
//             gen_candle(125.0, 7),
//             gen_candle(105.0, 8),
//             gen_candle(93.0, 9),
//             fail_long.clone(),
//             gen_candle(93.0, 11),
//             gen_candle(80.0, 12),
//             fail_short.clone(),
//             gen_candle(85.0, 14),
//             gen_candle(91.0, 15),
//         ];
//
//         let resolution_strategy = ResolutionStrategy::DynamicPivot(DynamicPivotResolution::new());
//
//         let setups = vec![
//             Setup {
//                 candle: long_setup,
//                 ticker: "TEST".to_string(),
//                 take_profit: Some(150.0),
//                 stop_loss: Some(95.0),
//                 interval: Interval::Day1,
//                 orientation: StrategyOrientation::Long,
//                 resolution_strategy: Some(resolution_strategy.clone()),
//             },
//             Setup {
//                 candle: short_setup,
//                 ticker: "TEST".to_string(),
//                 take_profit: Some(105.0),
//                 stop_loss: Some(155.0),
//                 interval: Interval::Day1,
//                 orientation: StrategyOrientation::Short,
//                 resolution_strategy: Some(resolution_strategy.clone()),
//             },
//             Setup {
//                 candle: fail_long,
//                 ticker: "TEST".to_string(),
//                 take_profit: Some(130.0),
//                 stop_loss: Some(80.0),
//                 interval: Interval::Day1,
//                 orientation: StrategyOrientation::Long,
//                 resolution_strategy: Some(resolution_strategy.clone()),
//             },
//             Setup {
//                 candle: fail_short,
//                 ticker: "TEST".to_string(),
//                 take_profit: Some(50.0),
//                 stop_loss: Some(91.0),
//                 interval: Interval::Day1,
//                 orientation: StrategyOrientation::Short,
//                 resolution_strategy: Some(resolution_strategy.clone()),
//             },
//         ];
//
//         let results = test_setups(&setups, &candles);
//
//         assert_eq!(results.n_setups, 4);
//         assert_eq!(results.avg_win_bars, 3.0);
//         assert_eq!(results.avg_win, 0.4);
//         assert_eq!(results.accuracy, 0.5);
//         assert_eq!(results.avg_loss, -0.25);
//         assert_eq!(results.avg_loss_bars, 2.0);
//         assert!(results.wins_std - 0.14142135 < 0.001);
//         assert!(results.losses_std - 0.07071067 < 0.001);
//         assert!(results.win_bars_std - 1.41421356 < 0.001);
//         assert_eq!(results.loss_bars_std, 0.0);
//     }
//
//     fn gen_candle(val: f64, increment: i64) -> Candle {
//         Candle {
//             open: val,
//             low: val,
//             high: val,
//             close: val,
//             timestamp: Utc::now() + Duration::days(increment),
//             volume: 1000.0,
//             indicators: HashMap::new(),
//         }
//     }
// }
