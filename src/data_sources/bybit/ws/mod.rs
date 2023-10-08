use anyhow::Result;
use tokio_tungstenite::connect_async;

use crate::models::websockets::{wsclient::WebsocketClient, subject::Subject, websocketpayload::WebsocketPayload};

pub async fn connect_ws(client: &WebsocketClient) -> Result<()> {
    let url = "wss://stream.bybit.com/realtime";
    let (mut ws_stream, _) = connect_async(url).await?;

    // let outgoing = OutgoingMessage::new("subscribe", "candles", "trade:1m:tBTCUSD");

    // let message = Message::Text(outgoing.to_string());
    // ws_stream.send(message).await?;

    let mut i = 0;

    // while let Some(msg) = ws_stream.next().await {
    loop {
        // let msg = msg?;

        // if let Message::Text(txt) = msg {
        //     let v: serde_json::Value = serde_json::from_str(txt.as_str())?;
        //     println!("({}) Value:{:#?}", i, v);
        // }

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