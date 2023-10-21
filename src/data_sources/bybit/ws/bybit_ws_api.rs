use crate::models::websockets::{websocket_payload::WebsocketPayload, wsclient::WebsocketClient};
use actix::Addr;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::{Error, Message};

use super::{
    incoming_message::{IncomingMessage, KlineResponse},
    outgoing_message::{OutgoingMessage, OutgoingMessageArg},
};

pub struct BybitWebsocketApi {
    _connection_id: String,
    client: Addr<WebsocketClient>,
}

impl BybitWebsocketApi {
    pub fn new(client: &Addr<WebsocketClient>) -> Self {
        Self {
            _connection_id: "".to_string(),
            client: client.clone(),
        }
    }

    pub async fn connect(&self) -> Result<()> {
        let url = "wss://stream-testnet.bybit.com/v5/public/spot";
        let (mut ws_stream, _) = connect_async(url).await?;

        self.send_ping(&mut ws_stream).await?;
        self.subscribe_to_kline(&mut ws_stream).await?;

        while let Some(msg) = ws_stream.next().await {
            BybitWebsocketApi::handle_message(msg, &self.client).await?;
        }

        ws_stream.close(None).await?;
        Ok(())
    }

    async fn send_ping(
        &self,
        ws_stream: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> Result<()> {
        let ping = OutgoingMessage::ping();
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
        msg: Result<Message, Error>,
        client: &Addr<WebsocketClient>,
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
}
