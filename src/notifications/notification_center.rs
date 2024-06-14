use crate::models::{setups::setup::Setup, traits::trading_strategy::TradingStrategy};
use anyhow::{anyhow, Result};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use std::env;

pub struct NotificationCenter;

impl NotificationCenter {
    pub async fn notify(setup: &Setup, strategy: &Box<dyn TradingStrategy>) -> Result<()> {
        Self::notify_email(setup, strategy).await?;

        Ok(())
    }

    pub async fn notify_email(setup: &Setup, strategy: &Box<dyn TradingStrategy>) -> Result<()> {
        let sender = env::var("EMAIL_SENDER")?;
        let sender = format!("{}", sender);
        let receiver = env::var("EMAIL_RECEIVER")?;
        let receiver = format!("{}", receiver);
        let username = env::var("EMAIL_LOGIN_USERNAME")?;
        let password = env::var("EMAIL_LOGIN_PASSWORD")?;

        let email = Message::builder()
            .from(sender.parse().unwrap())
            .to(receiver.parse().unwrap())
            .subject("Trade notification!")
            .header(ContentType::TEXT_PLAIN)
            .body(Self::get_body(setup, strategy));

        if let Ok(email) = email {
            let credentials = Credentials::new(username, password);
            let mailer = SmtpTransport::relay("smtp.gmail.com")?
                .credentials(credentials)
                .build();

            match mailer.send(&email) {
                Ok(_) => {
                    println!("Email sent successfully!");
                    Ok(())
                }
                Err(e) => Err(anyhow!(e)),
            }
        } else {
            Err(anyhow!("Unable to create notification email."))
        }
    }

    fn get_body(setup: &Setup, strategy: &Box<dyn TradingStrategy>) -> String {
        let s = format!(
            r#"Trade Notification:

        Strategy: {}
        Symbol: {}
        Date: {}
        Timeframe: {}
        Orientation: {}
        Suggested entry: {}

    Best of luck,
    Rust-Bot"#,
            strategy,
            setup.symbol,
            setup.candle.timestamp,
            setup.interval,
            setup.orientation,
            setup.candle.close,
        );
        s.to_string()
    }
}
