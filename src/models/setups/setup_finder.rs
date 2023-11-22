use actix::{Addr, Actor, Context, Handler, AsyncContext, fut::wrap_future};
use crate::models::{traits::trading_strategy::TradingStrategy, timeseries::TimeSeries, message_payloads::{candle_added_payload::CandleAddedPayload, request_latest_candles_payload::RequestLatestCandlesPayload}};

#[allow(dead_code)]
#[derive(Debug)]
pub struct SetupFinder {
    strategy: Box<dyn TradingStrategy>,
    ts: Addr<TimeSeries>,
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

        let ts = self.ts.clone();
        let fut = async move {
            let candles = ts.send(payload).await;

            match candles {
                Ok(candles) => {
                    // TODO: Check for setups
                    println!("Candles{:#?}", candles);
                },
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        };

        let actor_fut = wrap_future::<_, Self>(fut);
        ctx.wait(actor_fut);
    }
}

#[allow(dead_code)]
impl SetupFinder {
    pub fn new(strategy: Box<dyn TradingStrategy>, ts: Addr<TimeSeries>) -> Self {
        SetupFinder {
            strategy,
            ts,
        }
    }
}