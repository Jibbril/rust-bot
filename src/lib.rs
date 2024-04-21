mod data_sources;
mod indicators;
mod models;
mod notifications;
mod resolution_strategies;
mod strategy_testing;
mod trading_strategies;
mod utils;

use crate::{
    data_sources::bybit::rest::bybit_rest_api::BybitRestApi,
    indicators::{
        atr::ATR, populates_candles::PopulatesCandles, rsi::RSI,
        stochastic::Stochastic,
    },
    models::{net_version::NetVersion, websockets::wsclient::WebsocketClient},
    notifications::notification_center::NotificationCenter,
    trading_strategies::private::jb_2::JB2,
    utils::save_setups,
};
use actix::Actor;
use anyhow::Result;
use data_sources::{datasource::DataSource, local};
use dotenv::dotenv;
use indicators::{indicator_type::IndicatorType, populates_candles::PopulatesCandlesWithSelf};
use models::{
    candle::Candle,
    interval::Interval,
    message_payloads::{
        ts_subscribe_payload::TSSubscribePayload, websocket_payload::WebsocketPayload,
    },
    setups::setup_finder_builder::SetupFinderBuilder,
    strategy_orientation::StrategyOrientation,
    timeseries::TimeSeries,
    timeseries_builder::TimeSeriesBuilder,
    traits::trading_strategy::TradingStrategy,
};
use strategy_testing::strategy_tester::StrategyTester;
use tokio::time::{sleep, Duration};
use trading_strategies::{private::jb_1::JB1, public::rsi_basic::RsiBasic};

pub async fn run_dummy() -> Result<()> {
    todo!()
}

pub async fn run_market_buy() -> Result<()> {
    let net = NetVersion::Mainnet;
    let time = BybitRestApi::get_server_time(&net).await?;

    println!("Time: {:#?}", time);

    let wallet = BybitRestApi::get_wallet_balance(&net).await?;

    println!("Wallet: {:#?}", wallet);

    let buy = false;

    if buy {
        let balance: f64 = wallet.total_available_balance;
        BybitRestApi::market_buy(balance * 0.5, &net).await?;
    } else {
        BybitRestApi::market_sell_all(&wallet, &net).await?;
    }

    Ok(())
}

pub async fn run_single_indicator() -> Result<()> {
    let (k_len, k_smoothing, d_smoothing) = Stochastic::krown_args().stochastic_res()?;
    let indicator_type = IndicatorType::Stochastic(k_len, k_smoothing, d_smoothing);

    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;
    let needed_candles = k_len + k_smoothing + d_smoothing - 2;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, needed_candles + 500, &net)
        .await?;

    indicator_type.populate_candles(&mut ts)?;
    println!("Ts:{:#?}", ts);

    let mut client = WebsocketClient::new(source, interval, net);
    let addr = ts.start();

    client.add_observer(addr);
    client.start();

    // TODO: Enable check for whether new setups have arisen from updated indicators
    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_strategy() -> Result<()> {
    let strategy: Box<dyn TradingStrategy> = Box::new(JB1::new());
    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;

    // Initialize timeseries and indicators
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, strategy.min_length() + 300, &net)
        .await?;

    // ts.save_to_local(&source).await?;
    // let ts = source.load_local_data(symbol, &interval).await?;

    for indicator_type in strategy.required_indicators() {
        ts.add_indicator(indicator_type)?;
    }

    let ts_addr = ts.start();

    // Create setup finder and subscribe to timeseries
    let setup_finder = SetupFinderBuilder::new()
        .strategy(strategy)
        .ts(ts_addr.clone())
        .notifications_enabled(true)
        .live_trading_enabled(false)
        .build()?;

    let sf_addr = setup_finder.start();

    let payload = TSSubscribePayload {
        observer: sf_addr.clone(),
    };

    ts_addr.do_send(payload);

    // Start websocket client
    let mut wsclient = WebsocketClient::new(source, interval, net);
    wsclient.add_observer(ts_addr);
    wsclient.start();

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_double_strategies() -> Result<()> {
    let short_strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_args(
        14,
        45.0,
        55.0,
        StrategyOrientation::Short,
    ));
    let long_strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new_args(
        14,
        45.0,
        55.0,
        StrategyOrientation::Long,
    ));
    let interval = Interval::Minute1;
    let source = DataSource::Bybit;
    let net = NetVersion::Mainnet;

    // Initialize timeseries and indicators
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, long_strategy.min_length() + 300, &net)
        .await?;
    // ts.save_to_local(&source).await?;
    // let ts = source.load_local_data(symbol, &interval).await?;

    for indicator_type in long_strategy.required_indicators() {
        ts.add_indicator(indicator_type)?;
    }

    let ts_addr = ts.start();

    // Create setup finder and subscribe to timeseries
    let long_setup_finder = SetupFinderBuilder::new()
        .strategy(long_strategy)
        .ts(ts_addr.clone())
        .notifications_enabled(true)
        .build()?;
    let short_setup_finder = SetupFinderBuilder::new()
        .strategy(short_strategy)
        .ts(ts_addr.clone())
        .notifications_enabled(true)
        .build()?;

    let long_sf_addr = long_setup_finder.start();
    let short_sf_addr = short_setup_finder.start();

    let long_payload = TSSubscribePayload {
        observer: long_sf_addr.clone(),
    };

    let short_payload = TSSubscribePayload {
        observer: short_sf_addr.clone(),
    };

    ts_addr.do_send(long_payload);
    ts_addr.do_send(short_payload);

    // Start websocket client
    let mut wsclient = WebsocketClient::new(source, interval, net);
    wsclient.add_observer(ts_addr);
    wsclient.start();

    loop {
        sleep(Duration::from_secs(1)).await;
    }
}

pub async fn run_historical() -> Result<()> {
    dotenv().ok();

    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Day1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 19, &net)
        .await?;

    RSI::populate_candles(&mut ts)?;

    println!("Candles:{:#?}", ts.candles);

    Ok(())
}

pub async fn run_local() -> Result<()> {
    let mut ts = local::read_dummy_data("src/utils/data/atr_dummy_data.csv").await?;
    ATR::populate_candles(&mut ts)?;

    println!("{:#?}", ts);

    Ok(())
}

pub async fn run_setup_finder() -> Result<()> {
    let strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new());
    let ts = TimeSeries::dummy();
    let ts = ts.start();

    let sf = SetupFinderBuilder::new()
        .strategy(strategy)
        .ts(ts.clone())
        .notifications_enabled(true)
        .build()?;

    let sf = sf.start();

    let payload = TSSubscribePayload {
        observer: sf.clone(),
    };
    ts.do_send(payload);

    let mut i = 0.0;
    loop {
        sleep(Duration::from_secs(2)).await;
        let candle = Candle::dummy_from_val(i * 10.0);

        let payload = WebsocketPayload {
            ok: true,
            message: None,
            candle: Some(candle),
        };

        ts.do_send(payload);
        i += 1.0;
    }
}

pub async fn run_manual_setups() -> Result<()> {
    let mut ts = TimeSeriesBuilder::new()
        .symbol("BTCUSDT".to_string())
        .interval(Interval::Day1)
        .build();
    RSI::populate_candles(&mut ts)?;

    let strategy: Box<dyn TradingStrategy> = Box::new(RsiBasic::new());
    let ts = ts.start();

    let sf = SetupFinderBuilder::new()
        .strategy(strategy)
        .ts(ts.clone())
        .notifications_enabled(true)
        .build()?;
    let sf = sf.start();

    let payload = TSSubscribePayload {
        observer: sf.clone(),
    };
    ts.do_send(payload);

    let mut candles = Candle::dummy_data(20, "positive", 100.0);
    candles.extend(Candle::dummy_data(15, "negative", 300.0));

    for candle in candles {
        sleep(Duration::from_millis(500)).await;

        let payload = WebsocketPayload {
            ok: true,
            message: None,
            candle: Some(candle),
        };

        ts.do_send(payload);
    }

    Ok(())
}

pub async fn run_strategy_tester() -> Result<()> {
    // Get TimeSeries data
    let source = DataSource::Bybit;
    let strategy: Box<dyn TradingStrategy> = Box::new(JB2::new());
    let interval = strategy.interval();
    let net = NetVersion::Mainnet;

    println!("Fetching Timeseries data.");
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 20000, &net)
        .await?;

    // Calculate indicators for TimeSeries
    // Implement Strategy to analyze TimeSeries

    println!("Starting indicator calculations.");
    for indicator in strategy.required_indicators() {
        println!("Populating indicator: {:#?}", indicator);
        indicator.populate_candles(&mut ts)?;
    }

    let result = StrategyTester::test_strategy(&strategy, &ts.candles[300..])?;

    println!("{:#?}", result);

    Ok(())
}

pub async fn _run_strategy_testing() -> Result<()> {
    // Get TimeSeries data
    let source = DataSource::Bybit;
    let interval = Interval::Hour1;
    let net = NetVersion::Mainnet;
    let mut ts = source
        .get_historical_data("BTCUSDT", &interval, 1000, &net)
        .await?;

    // Calculate indicators for TimeSeries

    // Implement Strategy to analyze TimeSeries
    let strategy: Box<dyn TradingStrategy> = Box::new(JB1::new());

    for indicator in strategy.required_indicators() {
        indicator.populate_candles(&mut ts)?;
    }

    let setups = strategy.find_setups(&ts)?;

    let filename = format!("{}-setups.csv", strategy);
    save_setups(&setups, &filename)?;

    // Send email notifications
    if false {
        NotificationCenter::notify(&setups[0], &strategy).await?;
    }

    // Test results of taking setups
    // let result = test_setups(&setups, &ts.candles);

    Ok(())
}
