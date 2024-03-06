use super::strategy_test_result::StrategyTestResult;
use crate::{
    models::{candle::Candle, interval::Interval, traits::trading_strategy::TradingStrategy},
    resolution_strategies::{
        is_resolution_strategy::IsResolutionStrategy, resolution_strategy::ResolutionStrategy,
    },
    strategy_testing::strategy_test_result_builder::StrategyTestResultBuilder,
};
use anyhow::{Context, Result};

#[allow(dead_code)]
pub struct StrategyTester;

impl StrategyTester {
    #[allow(dead_code)]
    pub fn test_strategy(
        strat: &Box<dyn TradingStrategy>,
        candles: &[Candle],
    ) -> Result<StrategyTestResult> {
        let orientation = strat.orientation();
        let needed_candles = strat.candles_needed_for_setup();
        let mut result_builder = StrategyTestResultBuilder::new();
        let mut next_i = 0;

        println!("Starting Strategy test for {}", strat);

        // Loop over the needed candles to determine a setup and gather results.
        for (i, window) in candles.windows(needed_candles).enumerate() {
            // Bump index up to compensate for window size
            let i = i + needed_candles;

            if i % 1000 == 0 {
                println!("Testing Iteration {:#?}", i);
            }

            if i < needed_candles || i < next_i {
                continue;
            }

            let sb = strat.check_last_for_setup(&window);

            if sb.is_none() {
                continue;
            };

            let sb = sb.context("Expected SetupBuilder.")?;
            let setup = sb.symbol("TESTING").interval(&Interval::Day1).build()?;

            // Initialize resolution strategy
            let mut resolution_strategy = strat.default_resolution_strategy();
            resolution_strategy.set_initial_values(&setup)?;
            let mut n_bars = 0;

            // Loop over upcoming candles to determine outcome of setup
            loop {
                n_bars += 1;

                // Max number of bars in a setup to avoid infinite loops
                if n_bars > 100 {
                    break;
                };

                let end = i + n_bars;
                if end > candles.len() {
                    break;
                };

                let tp_candles_needed = resolution_strategy.n_candles_take_profit();
                let tp_candles = &candles[end - tp_candles_needed..end];
                let take_profit_reached =
                    resolution_strategy.take_profit_reached(&orientation, tp_candles)?;

                if take_profit_reached {
                    let increase =
                        tp_candles[tp_candles.len() - 1].close / setup.candle.close - 1.0;
                    result_builder.add_outcome(increase, n_bars);

                    break;
                }

                let sl_candles_needed = resolution_strategy.n_candles_stop_loss();
                let sl_candles = &candles[end - sl_candles_needed..end];
                let stop_loss_reached =
                    resolution_strategy.stop_loss_reached(&orientation, sl_candles)?;

                if stop_loss_reached {
                    let decrease =
                        sl_candles[sl_candles.len() - 1].close / setup.candle.close - 1.0;
                    result_builder.add_outcome(decrease, n_bars);
                    break;
                }
            }

            // Avoid scenarios where the same strategy can trigger new setups
            // while there is already a setup playing out.
            next_i = i + n_bars;
        }

        println!("Strategy testing complete, results:",);
        Ok(result_builder.build())
    }

    #[allow(dead_code)]
    pub fn by_strategies(
        _trading_strat: &Box<dyn TradingStrategy>,
        _resolution_strat: &ResolutionStrategy,
        _candles: &[Candle],
    ) -> StrategyTestResult {
        todo!()
    }
}
