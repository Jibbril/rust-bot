use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub total_available_balance: f64,
    pub coins: HashMap<String, WalletCoin>,
}

#[derive(Debug, Clone)]
pub struct WalletCoin {
    pub symbol: String,
    pub quantity: f64,
    pub usd_value: f64,
}

impl WalletCoin {
    pub fn new(symbol: &str, quantity: f64, usd_value: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            quantity,
            usd_value,
        }
    }
}
