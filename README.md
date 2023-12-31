# Rust-based trading bot 
This repo contains a rust-based trading/financial-analysis bot. It leverages [Rust](https://www.rust-lang.org/) and the [Actix actor model](https://actix.rs/docs/actix) to enable data gathering, financial analysis and (soon) trade management for various quantitative trading strategies. 

## Available Functionality
- Gathering financial data from various apis both historical and live through websockets.
- Calculations and management of various financial indicators based on candle data.
- Ability to define custom trading strategies based on whatever indicators/financial conditions the use can conjure up as well as backtesting of these.
- Integrated notification system/trade monitoring which messages the user (via email/sms) whenever a setup has emerged for a selected trading strategy.

## Roadmap
- Integrate trade management through Bybit.
- Construct UI to simplify usage
