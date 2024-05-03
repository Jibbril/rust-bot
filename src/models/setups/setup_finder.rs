use crate::{
    models::{
        message_payloads::{
            candle_added_payload::CandleAddedPayload,
            request_latest_candles_payload::RequestLatestCandlesPayload,
        },
        timeseries::TimeSeries,
        traits::trading_strategy::TradingStrategy,
        trade::Trade,
        setups::setup_finder_builder::SetupFinderBuilder, trade_builder::TradeBuilder
    },
    notifications::notification_center::NotificationCenter, data_sources::datasource::DataSource,
};
use actix::{fut::wrap_future, Actor, Addr, AsyncContext, Context, Handler};
use anyhow::Result;
use tokio::try_join;

#[derive(Debug)]
pub struct SetupFinder {
    symbol: String,
    strategy: Box<dyn TradingStrategy>,
    ts_addr: Addr<TimeSeries>,
    source: DataSource,
    notifications_enabled: bool,
    live_trading_enabled: bool,
    spawned_trade_addrs: Vec<Addr<Trade>>
}

impl Actor for SetupFinder {
    type Context = Context<Self>;
}

impl Handler<CandleAddedPayload> for SetupFinder {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, ctx: &mut Context<Self>) -> Self::Result {
        let payload = RequestLatestCandlesPayload {
            n: self.strategy.candles_needed_for_setup(),
        };

        let ts = self.ts_addr.clone();
        let strategy = self.strategy.clone_box();
        let notifications_enabled = self.notifications_enabled;
        let live_trading_enabled = self.live_trading_enabled;
        let mut spawned_trades = self.spawned_trade_addrs.clone();
        let source = self.source.clone();
        let symbol = self.symbol.clone();

        let fut = async move {
            let send_result = match ts.send(payload).await {
                Ok(res) => res,
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };

            // TODO: Seems actix result types for the RequestLatestCandlesPayload
            // are causing this "double" result. Investigate to see if there is
            // some way of removing.
            let candle_response = match send_result {
                Ok(res) => res,
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };

            let sb = strategy.check_last_for_setup(&candle_response.candles);

            if sb.is_none() {
                println!("No setup found");
                return;
            }

            let sb = sb.unwrap();
            let resolution_strategy = strategy.default_resolution_strategy();
            let setup = sb
                .symbol(&candle_response.symbol)
                .interval(&candle_response.interval)
                .resolution_strategy(&resolution_strategy)
                .build();

            let setup = match setup {
                Ok(setup) => setup,
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };

            println!("Setup found: {:#?}", setup);

            if live_trading_enabled {
                // Don't allow multiple active trades from the same strategy 
                // and timeseries
                if spawned_trades.len() > 0 {
                    return;
                }

                let wallet_fut = source.get_wallet();
                let last_price_fut = source.get_symbol_price(&symbol);

                let (wallet, last_price) = try_join!(wallet_fut, last_price_fut)
                    .expect("Unable to fetch data when creating Trade.");

                let quantity = wallet.total_available_balance / 2.0;
                let dollar_value = quantity * last_price;

                let trade = TradeBuilder::new()
                    .symbol(setup.symbol.clone())
                    .quantity(quantity)
                    .dollar_value(dollar_value)
                    .source(source)
                    .notifications_enabled(notifications_enabled)
                    .trading_enabled(true)
                    .resolution_strategy(resolution_strategy)
                    .orientation(strategy.orientation())
                    .timeseries_addr(ts)
                    .build()
                    .expect("Unable to build Trade in SetupFinder");

                let trade_addr = trade.start();

                spawned_trades.push(trade_addr)
            }

            if notifications_enabled {
                match NotificationCenter::notify(&setup, &strategy).await {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Error when notifying: {:#?}", e);
                        return;
                    }
                };
            }
        };

        let actor_fut = wrap_future::<_, Self>(fut);
        ctx.wait(actor_fut);
    }
}

#[allow(dead_code)]
impl SetupFinder {
    pub fn new(
        strategy: Box<dyn TradingStrategy>, 
        ts: Addr<TimeSeries>, 
        notifications_enabled: bool, 
        live_trading_enabled: bool, 
        spawned_trades: &[Addr<Trade>],
        source: DataSource
    ) -> Result<Self> {
        SetupFinderBuilder::new()
            .strategy(strategy)
            .ts(ts)
            .notifications_enabled(notifications_enabled)
            .live_trading_enabled(live_trading_enabled)
            .spawned_trades(spawned_trades)
            .source(source)
            .build()
    }
}
