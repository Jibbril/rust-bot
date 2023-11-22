mod outgoing_message;

use actix::Addr;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::models::{
    interval::Interval,
    websockets::wsclient::WebsocketClient, message_payloads::websocket_payload::WebsocketPayload,
};
use outgoing_message::OutgoingMessage;

#[allow(dead_code)]
pub async fn connect_ws(client: &Addr<WebsocketClient>, interval: &Interval) -> Result<()> {
    let url = "wss://api-pub.bitfinex.com/ws/2";
    let (mut ws_stream, _) = connect_async(url).await?;

    let interval = get_interval(interval);
    let key = format!("trade:{}:tBTCUSD", interval);
    let outgoing = OutgoingMessage::new("subscribe", "candles", &key);

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

fn get_interval(interval: &Interval) -> &str {
    match interval {
        Interval::Minute1 => "1m",
        Interval::Minute5 => "5m",
        _ => panic!("Not implemented"),
        // Interval::Minute15 => "15m",
        // Interval::Minute30 => "30m",
        // Interval::Hour1 => "1h",
        // Interval::Hour4 => "4h",
        // Interval::Hour12 => "12h",
        // Interval::Day1 => "1D",
        // Interval::Day5 => "5D",
        // Interval::Week1 => "7D",
    }
}
