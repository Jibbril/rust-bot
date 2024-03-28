use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{
    wallet::{Wallet, WalletCoin},
    wallet_builder::WalletBuilder,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBalanceResponse {
    #[serde(rename = "retCode")]
    ret_code: u32,

    #[serde(rename = "retMsg")]
    ret_msg: String,

    pub result: Option<WalletBalanceResult>,

    #[serde(rename = "retExtInfo")]
    ret_ext_info: HashMap<String, serde_json::Value>,

    time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletBalanceResult {
    pub list: Vec<WalletBalance>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletBalance {
    #[serde(rename = "totalEquity")]
    total_equity: String,

    #[serde(rename = "accountIMRate")]
    account_im_rate: String,

    #[serde(rename = "totalMarginBalance")]
    total_margin_balance: String,

    #[serde(rename = "totalInitialMargin")]
    total_initial_margin: String,

    #[serde(rename = "accountType")]
    account_type: String,

    #[serde(rename = "totalAvailableBalance")]
    pub total_available_balance: String,

    #[serde(rename = "accountMMRate")]
    account_mm_rate: String,

    #[serde(rename = "totalPerpUPL")]
    total_perp_upl: String,

    #[serde(rename = "totalWalletBalance")]
    total_wallet_balance: String,

    #[serde(rename = "accountLTV")]
    account_ltv: String,

    #[serde(rename = "totalMaintenanceMargin")]
    total_maintenance_margin: String,

    pub coin: Vec<CoinInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinInfo {
    #[serde(rename = "availableToBorrow")]
    available_to_borrow: String,

    bonus: String,

    #[serde(rename = "accruedInterest")]
    accrued_interest: String,

    #[serde(rename = "availableToWithdraw")]
    available_to_withdraw: String,

    #[serde(rename = "totalOrderIM")]
    total_order_im: String,

    equity: String,

    #[serde(rename = "totalPositionMM")]
    total_position_mm: String,

    #[serde(rename = "usdValue")]
    pub usd_value: String,

    #[serde(rename = "spotHedgingQty")]
    spot_hedging_qty: String,

    #[serde(rename = "unrealisedPnl")]
    unrealised_pnl: String,

    #[serde(rename = "collateralSwitch")]
    collateral_switch: bool,

    #[serde(rename = "borrowAmount")]
    borrow_amount: String,

    #[serde(rename = "totalPositionIM")]
    total_position_im: String,

    #[serde(rename = "walletBalance")]
    pub wallet_balance: String,

    #[serde(rename = "cumRealisedPnl")]
    cum_realised_pnl: String,

    locked: String,

    #[serde(rename = "marginCollateral")]
    margin_collateral: bool,

    pub coin: String,
}

impl WalletBalance {
    pub fn to_wallet(&self) -> Result<Wallet> {
        let wb = WalletBuilder::new();

        let mut coins: Vec<WalletCoin> = vec![];
        for c in self.coin.iter() {
            let quantity: f64 = c.wallet_balance.parse()?;
            let usd_value: f64 = c.usd_value.parse()?;
            let wc = WalletCoin::new(&c.coin, quantity, usd_value);
            coins.push(wc);
        }

        let wallet = wb
            .total_available_balance(self.total_available_balance.parse()?)
            .add_coins(coins)
            .build();

        Ok(wallet)
    }
}
