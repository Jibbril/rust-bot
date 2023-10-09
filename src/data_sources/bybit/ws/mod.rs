mod outgoing_message;
mod incoming_message;

use anyhow::{Result, anyhow};
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use crate::{models::websockets::{websocketpayload::WebsocketPayload, wsclient::WebsocketClient, subject::Subject,
}, data_sources::bybit::ws::{outgoing_message::{OutgoingMessage, OutgoingMessageArg}, incoming_message::IncomingMessage}};

pub async fn connect_ws(client: &WebsocketClient) -> Result<()> {
    let url = "wss://stream-testnet.bybit.com/v5/public/spot";
    let (mut ws_stream, _) = connect_async(url).await?;

    // Send ping
    let ping = OutgoingMessage::ping();
    let message = Message::Text(ping.to_json());
    ws_stream.send(message).await?;

    // Subscribe to kline
    let args = vec![
        OutgoingMessageArg {
            stream: "kline".to_string(),
            interval: "1".to_string(),
            symbol: "BTCUSDT".to_string()
        }
    ];
    let sub = OutgoingMessage::new("subscribe", args);
    let json = sub.to_json();

    ws_stream.send(Message::Text(json)).await?;

    let mut i = 0;
    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        if let Message::Text(txt) = msg {
            // let v: serde_json::Value = serde_json::from_str(txt.as_str())?;
            let parsed: IncomingMessage = serde_json::from_str(txt.as_str())?;

            match parsed {
                IncomingMessage::Pong(pong) => {
                    println!("Pong: {:#?}", pong);
                }
                IncomingMessage::Subscribe(sub) => {
                    println!("Subscribe: {:#?}", sub);
                }
                IncomingMessage::Kline(kline_response) => {
                    let kline = kline_response.get_kline()?;

                    if !kline.confirm { 
                        // TODO: Change to taking the next candle instead of the confirmed one. Solves issue with timestamps being wrong. 
                        continue; 
                    }

                    let candle = kline.to_candle()?;
                    let payload = WebsocketPayload {
                        ok: true,
                        message: Some(i.to_string()),
                        candle: Some(candle),
                    };

                    client.notify_observers(payload);
                }
            }
        }
    }

    println!("Done");

    ws_stream.close(None).await?;
    Ok(())
}