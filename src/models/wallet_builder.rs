use crate::models::wallet::{Wallet, WalletCoin};
use std::collections::HashMap;

pub struct WalletBuilder {
    total_available_balance: f64,
    coins: HashMap<String, WalletCoin>,
}

impl WalletBuilder {
    // Creates a new WalletBuilder with default values
    pub fn new() -> Self {
        WalletBuilder {
            total_available_balance: 0.0,
            coins: HashMap::new(),
        }
    }

    // Sets the total available balance
    pub fn total_available_balance(mut self, balance: f64) -> Self {
        self.total_available_balance = balance;
        self
    }

    pub fn add_coins(mut self, coins: Vec<WalletCoin>) -> Self {
        for coin in coins {
            self.coins.insert(coin.symbol.clone(), coin.clone());
        }
        self
    }

    // Finalizes the build and returns a Wallet
    pub fn build(self) -> Wallet {
        Wallet {
            total_available_balance: self.total_available_balance,
            coins: self.coins,
        }
    }
}
