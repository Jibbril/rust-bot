#[derive(Debug, Clone)]
#[allow(dead_code)] // TODO: Remove once used
pub struct StrategyTestResult {
    pub accuracy: f64,
    pub n_setups: usize,
    pub avg_profitability: f64,
    // TODO: Add normalized average profitability that takes the time Interval
    // into consideration. A 0.5% gain on the 15min chart may actually be much
    // better than a 5% gain on the monthly chart. Find some reasonable way to
    // compare these.
    pub avg_win: f64,
    pub avg_loss: f64,
    pub avg_win_bars: f64,
    pub avg_loss_bars: f64,
    pub wins_std: f64,
    pub losses_std: f64,
    pub win_bars_std: f64,
    pub loss_bars_std: f64,
    pub initial_account: f64,
    pub ending_account: f64,
}
