use crate::{
    data_sources::datasource::DataSource,
    models::{
        message_payloads::{
            candle_added_payload::CandleAddedPayload,
            request_latest_candles_payload::RequestLatestCandlesPayload, stop_payload::StopPayload,
            ping_payload::PingPayload,
        },
        setups::setup::Setup,
        timeseries::TimeSeries,
    },
    resolution_strategies::{
        is_resolution_strategy::IsResolutionStrategy, resolution_strategy::ResolutionStrategy,
    },
};
use actix::{Actor, ActorContext, Addr, AsyncContext, Context, Handler, WrapFuture};

#[derive(Debug)]
pub struct Trade {
    pub setup: Setup,
    pub quantity: f64,
    pub dollar_value: f64,
    pub source: DataSource,
    pub notifications_enabled: bool,
    pub trading_enabled: bool,
    pub resolution_strategy: ResolutionStrategy,
    pub timeseries: Addr<TimeSeries>,
}

impl Actor for Trade {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let source = self.source.clone();
        let symbol = self.setup.symbol.clone();
        let dollar_value = self.dollar_value.clone();
        self.resolution_strategy
            .set_initial_values(&self.setup)
            .expect("Unable to set initial values resolution strategy when starting Trade.");

        let fut = async move {
            let res = source.enter_trade(&symbol, dollar_value).await;

            match res {
                Ok(_) => println!("Successfully entered trade"),
                Err(e) => println!("Unable to enter trade, error: {:#?}", e),
            }

            // TODO: Setup system that manages scenario where Trade is unable
            // to enter. Send notifications and or stop service.
        };

        ctx.spawn(fut.into_actor(self));
    }
}

impl Handler<StopPayload> for Trade {
    type Result = ();

    fn handle(&mut self, _msg: StopPayload, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Handler<PingPayload> for Trade {
    type Result = ();

    fn handle(&mut self, _msg: PingPayload, _ctx: &mut Self::Context) -> Self::Result {}
}

impl Handler<CandleAddedPayload> for Trade {
    type Result = ();

    fn handle(&mut self, _msg: CandleAddedPayload, ctx: &mut Self::Context) -> Self::Result {
        let resolution_strategy = self.resolution_strategy.clone();
        let tp_candles_needed = resolution_strategy.n_candles_take_profit();
        let sl_candles_needed = resolution_strategy.n_candles_stop_loss();
        let orientation = self.setup.orientation.clone();
        let ts_addr = self.timeseries.clone();
        let source = self.source.clone();
        let symbol = self.setup.symbol.clone();
        let self_addr = ctx.address();

        // Multiply to avoid scenarios where quantity is slightly larger than
        // account balance (caused by sudden price changes in time between
        // account balance is checked and initial buy is performed).
        let quantity = self.quantity;

        let payload = RequestLatestCandlesPayload {
            n: tp_candles_needed.max(sl_candles_needed),
        };

        let fut = async move {
            let candle_response = ts_addr
                .send(payload)
                .await
                .expect("Unable to fetch timeseries data in Trade.")
                .expect("Unable to parse LatestCandleResponse in Trade.");

            let end = candle_response.candles.len();

            let tp_candles = &candle_response.candles[end - tp_candles_needed..end];
            let take_profit_reached = resolution_strategy
                .take_profit_reached(&orientation, tp_candles)
                .expect("Unable to perform take-profit check in Active Trade");

            let sl_candles = &candle_response.candles[end - sl_candles_needed..end];
            let stop_loss_reached = resolution_strategy
                .stop_loss_reached(&orientation, sl_candles)
                .expect("Unable to perform stop-loss check in Active Trade");

            if take_profit_reached || stop_loss_reached {
                let res = source.exit_trade(&symbol, quantity).await;

                match res {
                    Ok(_) => println!("Trade successfully exited!"),
                    Err(e) => println!("Trade exit failed with error: {:#?}", e),
                }
                // TODO: Handle/notify user in case selling was unsuccessful.

                self_addr.do_send(StopPayload);
            }
        };

        ctx.spawn(fut.into_actor(self));
    }
}
