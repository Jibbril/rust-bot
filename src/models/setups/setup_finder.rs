use crate::{
    data_sources::datasource::DataSource,
    models::{
        message_payloads::{
            candle_added_payload::CandleAddedPayload, ping_payload::PingPayload, request_latest_candles_payload::RequestLatestCandlesPayload, triggered_payload::TriggeredPayload, ts_subscribe_payload::TSSubscribePayload
        },
        timeseries::TimeSeries,
        trade::Trade,
        trade_builder::TradeBuilder,
        traits::trading_strategy::TradingStrategy,
    },
    notifications::notification_center::NotificationCenter,
};
use actix::{fut::wrap_future, Actor, Addr, AsyncContext, Context, Handler};
use anyhow::Result;
use tokio::try_join;

#[derive(Debug)]
pub struct SetupFinder {
    strategy: Box<dyn TradingStrategy>,
    ts_addr: Addr<TimeSeries>,
    source: DataSource,
    notifications_enabled: bool,
    live_trading_enabled: bool,
    only_trigger_once: bool,
    triggered: bool,
    spawned_trade_addrs: Vec<Addr<Trade>>,
}

impl Actor for SetupFinder {
    type Context = Context<Self>;
}

impl Handler<CandleAddedPayload> for SetupFinder {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, ctx: &mut Context<Self>) -> Self::Result {
        if self.only_trigger_once && self.triggered {
            return ();
        }

        let payload = RequestLatestCandlesPayload {
            n: self.strategy.candles_needed_for_setup(),
        };

        let self_addr = ctx.address();
        let ts = self.ts_addr.clone();
        let mut strategy = self.strategy.clone_box();
        let notifications_enabled = self.notifications_enabled;
        let live_trading_enabled = self.live_trading_enabled;
        let mut spawned_trades = self.spawned_trade_addrs.clone();
        let source = self.source.clone();

        // Clear trades before potentially starting new one
        self.clear_closed_trades();

        let fut = async move {
            let candle_response = ts.send(payload).await
                .unwrap_or_else(|e| panic!("Failed to send payload: {:#?}", e))
                .unwrap_or_else(|e| panic!("Failed to unwrap LatestCandleResponse: {:#?}", e));

            let sb = strategy.check_last_for_setup(&candle_response.candles);

            if sb.is_none() {
                return;
            }

            let sb = sb.unwrap();
            let resolution_strategy = strategy.default_resolution_strategy();
            let setup = sb
                .symbol(&candle_response.symbol)
                .interval(&candle_response.interval)
                .build();

            let setup = match setup {
                Ok(setup) => setup,
                Err(e) => {
                    println!("Error: {:#?}", e);
                    return;
                }
            };

            self_addr.do_send(TriggeredPayload);

            println!("Setup found: {:#?}", setup);

            if live_trading_enabled {
                if spawned_trades.len() > 0 {
                    return;
                }

                let wallet_fut = source.get_wallet();
                let last_price_fut = source.get_symbol_price(&setup.symbol);

                let (wallet, last_price) = try_join!(wallet_fut, last_price_fut)
                    .expect("Unable to fetch data when creating Trade.");

                // TODO: Implement system to enable variantions on position size
                // Quantity is half of available balance
                let dollar_value = wallet.total_available_balance / 2.0;
                let quantity = dollar_value / last_price;

                let trade = TradeBuilder::new()
                    .setup(setup.clone())
                    .quantity(quantity)
                    .dollar_value(dollar_value)
                    .source(source)
                    .notifications_enabled(notifications_enabled)
                    .trading_enabled(true)
                    .resolution_strategy(resolution_strategy)
                    .orientation(strategy.orientation())
                    .timeseries_addr(ts.clone())
                    .build()
                    .expect("Unable to build Trade in SetupFinder");

                let trade_addr = trade.start();

                // Subscribe Trade to TimeSeries so it receives updates when 
                // candles are added
                let ts_subscribe_payload = TSSubscribePayload {
                    observer: trade_addr.clone().recipient(),
                };

                ts.do_send(ts_subscribe_payload);

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

impl Handler<TriggeredPayload> for SetupFinder {
    type Result = ();

    fn handle(&mut self, _msg: TriggeredPayload, _ctx: &mut Self::Context) -> Self::Result {
        self.triggered = true;
    }
}

#[allow(dead_code)]
impl SetupFinder {
    pub fn new(
        strategy: Box<dyn TradingStrategy>,
        ts_addr: Addr<TimeSeries>,
        notifications_enabled: bool,
        live_trading_enabled: bool,
        only_trigger_once: bool,
        spawned_trade_addrs: &[Addr<Trade>],
        source: DataSource,
    ) -> Result<Self> {
        Ok(SetupFinder {
            strategy, 
            ts_addr,
            notifications_enabled,
            live_trading_enabled,
            only_trigger_once,
            spawned_trade_addrs: spawned_trade_addrs.to_vec(),
            source,
            triggered: false
        })
    }

    fn clear_closed_trades(&mut self) {
        let mut trade_addrs = vec![];

        for addr in &self.spawned_trade_addrs {
            let res = addr.try_send(PingPayload);

            if res.is_ok() {
                trade_addrs.push(addr.clone());
            }
        }

        self.spawned_trade_addrs = trade_addrs;
    }
}
