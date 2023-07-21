#[derive(Debug, Clone)]
#[allow(dead_code)] // TODO: Remove once used
pub struct TestResult {
    pub accuracy: f64,
    pub n: usize,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub avg_win_bars: f64,
    pub avg_loss_bars: f64,
}
