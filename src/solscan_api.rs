use actix_web::http::StatusCode;
use log::info;
use rust_decimal::prelude::*;


#[derive(Clone, Debug)]
pub struct SolanaTX {
    tx_signature: String,
    pub block_time: i64,
    source_owner: String,
    pub(crate) destination_owner: String,
    pub token_address: String,
    pub token_symbol: String,
    pub(crate) amount: f32,
    pub status: String,
}

pub async fn get_solana_tx(tx_signature: String) -> Result<SolanaTX, StatusCode> {
    let client = reqwest::Client::new();
    let url = "https://public-api.solscan.io/transaction/".to_owned() + tx_signature.as_str();
    let result = client
        .get(url.clone())
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .unwrap();

    info!("{}", url);
    match result.status() {
        StatusCode::OK => {
            let json = json::parse(result.text().await.unwrap().as_str()).unwrap();
            println!("{}", json);
            let tx = SolanaTX {
                tx_signature: json["txHash"].to_string(),
                block_time: json["blockTime"].to_string().parse::<i64>().unwrap(),
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
            Ok(tx)
        }
        _ => Err(result.status()),
    }
}
