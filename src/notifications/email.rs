use crate::{
    models::generic_result::GenericResult,
    trading_strategies::{setup::Setup, strategy::Strategy},
};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use std::env;

pub async fn notify_email(setup: &Setup, strategy: &Strategy) -> GenericResult<()> {
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
        .body(get_body(setup, strategy));

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
            Err(e) => Err(Box::new(e)),
        }
    } else {
        Err("Unable to create notification email.".into())
    }
}

fn get_body(setup: &Setup, strategy: &Strategy) -> String {
    let s = format!(
        r#"Trade Notification:

    Strategy: {}
    Ticker: {}
    Date: {}
    Timeframe: {}
    Orientation: {}
    Suggested entry: {}
    Suggested take profit: {} 
    Suggested stop loss: {}

Best of luck,
Winston"#,
        strategy,
        setup.ticker,
        setup.candle.timestamp,
        setup.interval,
        setup.orientation,
        setup.candle.close,
        setup.take_profit,
        setup.stop_loss
    );
    s.to_string()
}
