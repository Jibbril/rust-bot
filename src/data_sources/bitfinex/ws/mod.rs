mod outgoing_message;
/*

General approach:

1. Connect to websocket and enable calling a notify function whenever a new candle is received.
2. Create functionality to add a new candle and notify all listening TimeSeries.
3. Implement notify function for TimeSeries.
  a. Check if the latest existing candle is the one before the new one. If not, fetch historical data up to the new candle.
  b. Add the new candle to the TimeSeries.
  c. Save the new candle/candles to disk.
*/

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::models::websockets::{websocket_payload::WebsocketPayload, wsclient::WebsocketClient,
};
use outgoing_message::OutgoingMessage;

#[allow(dead_code)]
pub async fn connect_ws(_client: &WebsocketClient) -> Result<()> {
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
        }

        let _payload = WebsocketPayload {
            ok: true,
            message: Some(i.to_string()),
            candle: None,
        };

        // TODO: Implement actor model to notify observers

        i += 1;
        if i > 30 {
            break;
        };
    }

    println!("Done");

    ws_stream.close(None).await?;
    Ok(())
}
