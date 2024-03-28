use crate::models::net_version::NetVersion;
use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;

type HmacSha256 = Hmac<Sha256>;

pub fn bybit_url(path: &str, net: &NetVersion) -> String {
    match net {
        NetVersion::Mainnet => format!("https://api.bybit.com{}", path),
        NetVersion::Testnet => format!("https://api-testnet.bybit.com{}", path),
    }
}

pub fn generate_hmac_signature(
    timestamp: u64,
    api_key: &String,
    recv_window: i64,
    params: String,
) -> Result<String> {
    let mut mac = HmacSha256::new_from_slice(bybit_secret()?.as_bytes())?;
    mac.update(timestamp.to_string().as_bytes());
    mac.update(api_key.as_bytes());
    mac.update(recv_window.to_string().as_bytes());
    mac.update(params.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    Ok(hex::encode(code_bytes))
}

pub fn bybit_secret() -> Result<String> {
    Ok(env::var("BYBIT_API_SECRET")?)
}

pub fn bybit_key() -> Result<String> {
    Ok(env::var("BYBIT_API_KEY")?)
}
