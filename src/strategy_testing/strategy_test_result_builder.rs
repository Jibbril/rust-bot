use crate::utils::math::{sma, std};
use super::strategy_test_result::StrategyTestResult;

const INITIAL_ACCOUNT_SIZE: f64 = 100_000.0;

#[derive(Debug, Clone)]
#[allow(dead_code)] // TODO: Remove once used
pub struct StrategyTestResultBuilder {
    pub n_setups: usize,
    pub n_wins: usize,
    pub n_losses: usize,
    pub account_size: f64,
    pub wins: Vec<f64>,
    pub win_bars: Vec<usize>,
    pub losses: Vec<f64>,
    pub loss_bars: Vec<usize>,
}

impl StrategyTestResultBuilder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            n_setups: 0,
            n_wins: 0,
            n_losses: 0,
            account_size: INITIAL_ACCOUNT_SIZE,
            wins: Vec::new(),
            win_bars: Vec::new(),
            losses: Vec::new(),
            loss_bars: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_win(&mut self, increase: f64, n_bars: usize) {
        self.n_wins += 1;
        self.n_setups += 1;
        self.wins.push(increase);
        self.win_bars.push(n_bars);
        self.account_size += self.account_size * increase;
    }

    #[allow(dead_code)]
    pub fn add_loss(&mut self, drawdown: f64, n_bars: usize) {
        self.n_losses += 1;
        self.n_setups += 1;

        let drawdown = if drawdown <= 0.0 {
            drawdown
        } else {
            -drawdown
        };

        self.losses.push(drawdown);
        self.loss_bars.push(n_bars);
        self.account_size += self.account_size * drawdown;
    }

    #[allow(dead_code)]
    pub fn build(self) -> StrategyTestResult {
        let total_wins = self.wins.iter().sum::<f64>();
        let total_losses = self.losses.iter().sum::<f64>();

        let avg_win = if self.n_wins > 0 { 
            total_wins / self.n_wins as f64 
        } else { 0.0 };
        let avg_loss = if self.n_losses > 0 { 
            total_losses / self.n_losses as f64 
        } else { 0.0 };

        let avg_win_bars = if !self.win_bars.is_empty() { 
            self.win_bars.iter().sum::<usize>() as f64 / self.n_wins as f64 
        } else { 0.0 };
        let avg_loss_bars = if !self.loss_bars.is_empty() { 
            self.loss_bars.iter().sum::<usize>() as f64 / self.n_losses as f64 
        } else { 0.0 };

        let accuracy = if self.n_setups > 0 { 
            self.n_wins as f64 / self.n_setups as f64 
        } else { 0.0 };
        let avg_profitability = (total_wins + total_losses) / self.n_setups as f64;

        let f_win_bars: Vec<f64> = self.win_bars.iter()
            .map(|b| *b as f64)
            .collect();
        let f_loss_bars: Vec<f64> = self.loss_bars.iter()
            .map(|b| *b as f64)
            .collect();

        StrategyTestResult {
            accuracy,
            n_setups: self.n_setups,
            avg_profitability,
            avg_win,
            avg_loss,
            avg_win_bars,
            avg_loss_bars,
            initial_account: INITIAL_ACCOUNT_SIZE,
            ending_account: self.account_size,
            wins_std: std(&self.wins, sma(&self.wins)),
            losses_std: std(&self.losses, sma(&self.losses)), // Placeholder, calculate this
            win_bars_std: std(&f_win_bars, sma(&f_win_bars)), // Placeholder, calculate this
            loss_bars_std: std(&f_loss_bars, sma(&f_loss_bars)), // Placeholder, calculate this
        }
    }
}


