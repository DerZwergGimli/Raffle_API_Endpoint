use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

use super::Raffle;

pub async fn create_raffle_raffle_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "raffle_number": 1 })
        .options(options)
        .build();
    client
        .database("Raffle")
        .collection::<Raffle>("raffle")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

pub async fn create_raffle_ticket_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "title": 1 })
        .options(options)
        .build();
    client
        .database("Raffle")
        .collection::<Raffle>("raffle")
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}