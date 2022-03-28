use actix_web::http::StatusCode;
use actix_web::web::Json;
use json::Error as jError;
use json::JsonValue;
use log::info;
use reqwest::Error;
use rust_decimal::prelude::*;
use std::f32::consts::E;

#[derive(Clone, Debug)]
pub struct SolanaTX {
    tx_signature: String,
    source_owner: String,
    pub(crate) destination_owner: String,
    token_address: String,
    token_symbol: String,
    pub(crate) amount: f32,
    status: String,
}

pub async fn get_solana_tx(tx_signature: String) -> Result<SolanaTX, StatusCode> {
    let client = reqwest::Client::new();
    let url = "https://public-api.solscan.io/transaction/".to_owned() + tx_signature.as_str();
    let mut result = client
        .get(url.clone())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .unwrap();

    info!("{}", url);
    match result.status() {
        StatusCode::OK => {
            let mut json = json::parse(result.text().await.unwrap().as_str()).unwrap();
            println!("{}", json);
            let tx = SolanaTX {
                tx_signature: json["txHash"].to_string(),
                source_owner: json["tokenTransfers"][0]["source_owner"].to_string(),
                destination_owner: json["tokenTransfers"][0]["destination_owner"].to_string(),
                token_address: json["tokenTransfers"][0]["token"]["address"].to_string(),
                token_symbol: json["tokenTransfers"][0]["token"]["symbol"].to_string(),
                amount: Decimal::new(
                    json["tokenTransfers"][0]["amount"]
                        .to_string()
                        .parse::<i64>()
                        .unwrap_or_default(),
                    json["tokenTransfers"][0]["token"]["decimals"]
                        .to_string()
                        .parse::<u32>()
                        .unwrap_or_default(),
                )
                .to_f32()
                .unwrap(),
                status: json["status"].to_string(),
            };
            println!("{:?}", tx);
            Ok(tx.clone())
        }
        _ => Err(result.status()),
    }
}
