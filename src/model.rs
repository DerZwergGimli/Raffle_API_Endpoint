use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Raffle {
    #[serde(default)]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub description: String,
    pub ticket_amount: u16,
    pub ticket_price: f32,
    pub ticket_token_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Ticket {
    #[serde(default)]
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub raffle_id: ObjectId,
    pub username: String,
    pub spl_tx_signature: String,
    #[serde(default)]
    pub amount: u16,
}
