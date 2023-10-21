mod outgoing_message;

use actix::Addr;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::models::websockets::{websocket_payload::WebsocketPayload, wsclient::WebsocketClient};
use outgoing_message::OutgoingMessage;

#[allow(dead_code)]
pub async fn connect_ws(client: &Addr<WebsocketClient>) -> Result<()> {
    let url = "wss://api-pub.bitfinex.com/ws/2";
    let (mut ws_stream, _) = connect_async(url).await?;

    let outgoing = OutgoingMessage::new("subscribe", "candles", "trade:1m:tBTCUSD");

    let message = Message::Text(outgoing.to_string());
    ws_stream.send(message).await?;

    let mut i = 0;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        if let Message::Text(txt) = msg {
            let _v: serde_json::Value = serde_json::from_str(txt.as_str())?;
            // println!("({}) Value:{:#?}", i, v);
            // TODO: Implement conversion from JSON to Candle
        }

        let payload = WebsocketPayload {
            ok: true,
            message: Some(i.to_string()),
            candle: None,
        };

        client.do_send(payload);

        i += 1;
        if i > 30 {
            break;
        };
    }

    println!("Done");

    ws_stream.close(None).await?;
    Ok(())
}
