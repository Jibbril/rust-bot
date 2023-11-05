use crate::models::websockets::{websocket_payload::WebsocketPayload, wsclient::WebsocketClient};
use actix::{spawn, Addr};
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::TcpStream,
    select,
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
    time::{sleep, Duration},
    try_join,
};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::Message;

use super::{
    incoming_message::{IncomingMessage, KlineResponse},
    outgoing_message::{OutgoingMessage, OutgoingMessageArg},
};

pub struct BybitWebsocketApi {
    client: Addr<WebsocketClient>,
}

impl BybitWebsocketApi {
    pub fn new(client: &Addr<WebsocketClient>) -> Self {
        Self {
            client: client.clone(),
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let url = "wss://stream-testnet.bybit.com/v5/public/spot";
        let (mut ws_stream, _) = connect_async(url).await?;
        self.subscribe_to_kline(&mut ws_stream).await?;
        Self::send_ping(None, &mut ws_stream).await?;

        let (tx, rx) = channel(32);

        let ping_handle = Self::spawn_ping_task(tx).await;
        let client = self.client.clone();
        let message_handle = Self::spawn_message_task(client, ws_stream, rx).await;

        try_join!(ping_handle, message_handle)?;

        Ok(())
    }

    async fn send_ping(
        req_id: Option<String>,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let ping = OutgoingMessage::ping(req_id);
        let message = Message::Text(ping.to_json());

        ws_stream.send(message).await?;

        Ok(())
    }

    async fn subscribe_to_kline(
        &self,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let args = vec![OutgoingMessageArg {
            stream: "kline".to_string(),
            interval: "1".to_string(),
            symbol: "BTCUSDT".to_string(),
        }];
        let sub = OutgoingMessage::new("subscribe", args);
        let json = sub.to_json();

        ws_stream.send(Message::Text(json)).await?;

        Ok(())
    }

    async fn handle_message(
        client: &Addr<WebsocketClient>,
        msg: Result<Message, tungstenite::Error>,
    ) -> Result<()> {
        let msg = msg?;

        if let Message::Text(txt) = msg {
            // let v: serde_json::Value = serde_json::from_str(txt.as_str())?;
            let parsed: IncomingMessage = serde_json::from_str(txt.as_str())?;

            match parsed {
                IncomingMessage::Pong(pong) => {
                    println!("Pong: {:#?}", pong)
                }
                IncomingMessage::Subscribe(sub) => println!("Subscribe: {:#?}", sub),
                IncomingMessage::Kline(kline_response) => {
                    Self::handle_kline(kline_response, client).await?
                }
            }
        }

        Ok(())
    }

    async fn handle_kline(
        kline_response: KlineResponse,
        client: &Addr<WebsocketClient>,
    ) -> Result<()> {
        let kline = kline_response.get_kline()?;

        if kline.confirm {
            // TODO: Change to taking the next candle instead of the confirmed one. Solves issue with timestamps being wrong.
            let candle = kline.to_candle()?;
            let payload = WebsocketPayload {
                ok: true,
                message: None,
                candle: Some(candle),
            };

            client.do_send(payload);
        }

        Ok(())
    }

    async fn spawn_ping_task(tx: Sender<&'static str>) -> JoinHandle<()> {
        spawn(async move {
            loop {
                sleep(Duration::from_secs(20)).await;
                tx.send("ping").await.expect("Failed to send ping");
            }
        })
    }

    async fn spawn_message_task(
        client: Addr<WebsocketClient>,
        mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        mut rx: Receiver<&'static str>,
    ) -> JoinHandle<()> {
        spawn(async move {
            loop {
                select! {
                    _ = rx.recv() => {
                        if let Err(e) = Self::send_ping(None, &mut ws_stream).await {
                            println!("Error in Websockets: {:#?}",e);
                            break;
                        }
                    }
                    msg = ws_stream.next() => {
                        if let Some(msg) = msg {
                            match Self::handle_message(&client, msg).await {
                                Ok(_) => (),
                                Err(e) => {
                                    println!("Error in Websockets: {:#?}",e);
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
