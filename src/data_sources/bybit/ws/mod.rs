mod outgoing_message;

use anyhow::Result;
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::{models::websockets::{
    subject::Subject, websocketpayload::WebsocketPayload, wsclient::WebsocketClient,
}, data_sources::bybit::ws::outgoing_message::OutgoingMessage};

pub async fn connect_ws(client: &WebsocketClient) -> Result<()> {
    let url = "wss://stream-testnet.bybit.com/v5/public/spot";
    let (mut ws_stream, _) = connect_async(url).await?;

    // let outgoing = OutgoingMessage::new("subscribe", "candles", "trade:1m:tBTCUSD");

    let ping = OutgoingMessage::ping();

    let message: Message = Message::Text(ping.to_json());
    ws_stream.send(message).await?;

    let mut i = 0;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;

        if let Message::Text(txt) = msg {
            let v: serde_json::Value = serde_json::from_str(txt.as_str())?;
            println!("({}) Value:{:#?}", i, v);
        }

        let payload = WebsocketPayload {
            ok: true,
            message: Some(i.to_string()),
            candle: None,
        };

        client.notify_observers(payload);

        i += 1;
        if i > 30 {
            break;
        };
    }

    println!("Done");

    ws_stream.close(None).await?;
    Ok(())
}
