pub mod incoming_message;
pub mod outgoing_message;
pub mod action;
pub mod subscription; 

use std::env;
use anyhow::Result;
use dotenv::dotenv;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tungstenite::Message;
use subscription::Subscription;
use outgoing_message::OutgoingMessage; 
use incoming_message::IncomingMessage;
use action::Action;

#[allow(dead_code)]
pub async fn dummy_example() -> Result<()> {
    dotenv().ok();
    let api_key = env::var("CRYPTOCOMPARE_KEY")?;
    let url = format!("wss://streamer.cryptocompare.com/v2?api_key={}", api_key);
    let (mut ws_stream,_) = connect_async(url).await.expect("Failed to connect");

    let sub = Subscription::new("5", "CCCAGG", "BTC", "USD");
    let outgoing = OutgoingMessage::new(Action::SubAdd, vec![sub.clone()]);
    let message = Message::Text(outgoing.to_string());

    ws_stream.send(message).await?;

    let mut i = 0;
    // use stream to read data from websocket
    while let Some(msg) = ws_stream.next().await {
        let msg = msg.expect("Failed to get response");

        if let Message::Text(txt) = msg {
            let v: Value = serde_json::from_str(txt.as_str()).expect("Failed to parse");
            let parsed: Result<IncomingMessage, serde_json::Error> = serde_json::from_str(txt.as_str());
            
            if parsed.is_err() {
                println!("Value:{:#?}", v);
                println!("Parsed: {:#?}",parsed);
                let unsub = OutgoingMessage::new(Action::SubRemove, vec![sub.clone()]);
                let message = Message::Text(unsub.to_string());
                ws_stream.send(message).await?;            
            } else {
                println!("Handled correctly for iteration: {}",i);
            }
        }

        i += 1;
        if i > 10 {
            break;
        };
    }

    ws_stream.close(None).await?;

    Ok(())
}
