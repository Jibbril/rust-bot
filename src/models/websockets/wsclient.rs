use crate::{
    data_sources::datasource::DataSource,
    models::{
        interval::Interval, message_payloads::websocket_payload::WebsocketPayload,
        net_version::NetVersion, timeseries::TimeSeries,
    },
};
use actix::{Actor, Addr, AsyncContext, Context, Handler, WrapFuture};

pub struct WebsocketClient {
    source: DataSource,
    interval: Interval,
    observers: Vec<Addr<TimeSeries>>,
    net: NetVersion,
}

impl Actor for WebsocketClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let client = ctx.address();
        let source = self.source.clone();
        let interval = self.interval.clone();
        let net = self.net.clone();
        let fut = async move {
            if let Err(e) = source.connect_ws(client, interval, &net).await {
                // TODO: Add logic for error handling, restarting client etc.
                println!("Error: {}", e);
            }
        };

        ctx.spawn(fut.into_actor(self));
    }
}

impl Handler<WebsocketPayload> for WebsocketClient {
    type Result = ();

    fn handle(&mut self, payload: WebsocketPayload, _ctx: &mut Context<Self>) -> Self::Result {
        if payload.ok {
            for observer in &self.observers {
                observer.do_send(payload.clone());
            }
        } else {
            let err = match payload.message {
                Some(message) => message,
                None => "Unknown error".to_string(),
            };
            println!("Error: {}", err);
        }
    }
}

impl WebsocketClient {
    pub fn new(source: DataSource, interval: Interval, net: NetVersion) -> Self {
        Self {
            source,
            interval,
            net,
            observers: vec![],
        }
    }

    pub fn add_observer(&mut self, observer: Addr<TimeSeries>) {
        self.observers.push(observer);
    }
}
