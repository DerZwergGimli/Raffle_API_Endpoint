use crate::{ObjectId, Raffle, Ticket};
use futures::stream::{ TryStreamExt};
use lazy_static::lazy_static;
use mongodb::bson::{doc};
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Client};
use std::{env};

lazy_static! {
    static ref DB_NAME: String = env::var("DB_NAME").unwrap_or_else(|_| "DB_Raffle".to_string());
    static ref COLL_RAFFLE: String = env::var("COLL_RAFFLE").unwrap_or_else(|_| "Raffle".to_string());
    static ref COLL_TICKET: String = env::var("COLL_TICKET").unwrap_or_else(|_| "Ticket".to_string());
}

#[derive(Clone)]
pub struct DatabaseRaffle {

}

impl DatabaseRaffle {
    pub fn new() -> Self {
        Self {

        }
    }

    //region === INSERT ===
    pub async fn insert_raffle(
        &self,
        client: &Client,
        raffle: &mut Raffle,
    ) -> Result<InsertOneResult, Error> {
        raffle.date_created = chrono::Utc::now().timestamp();
        raffle.date_updated = chrono::Utc::now().timestamp();
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());
        raffle.id = ObjectId::new();
        raffle.status = "created".to_string();
        collection.insert_one(raffle, None).await
    }

    pub async fn insert_ticket(
        &self,
        client: &Client,
        ticket: &mut Ticket,
    ) -> Result<InsertOneResult, Error> {
        ticket.date_created = chrono::Utc::now().timestamp();
        ticket.date_updated = chrono::Utc::now().timestamp();
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());
        collection.insert_one(ticket, None).await
    }
    //endregion

    //region === REMOVE ===
    pub async fn remove_raffle(
        &self,
        client: &Client,
        raffle_id: ObjectId,
    ) -> Result<DeleteResult, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());
        collection.delete_one(doc! {"_id": raffle_id}, None).await
    }

    pub async fn remove_ticket(
        &self,
        client: &Client,
        ticket_id: ObjectId,
    ) -> Result<DeleteResult, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());
        collection.delete_one(doc! {"_id": ticket_id}, None).await
    }
    //endregion

    //region === FIND ALL ===
    pub async fn get_all_raffles(&self, client: &Client) -> Result<Vec<Raffle>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());
        collection.find(None, None).await?.try_collect().await
    }

    pub async fn get_all_tickets(&self, client: &Client) -> Result<Vec<Ticket>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());

        collection.find(None, None).await?.try_collect().await
    }
    //endregion

    //region === FIND BY ID ===
    pub async fn get_raffle_by_id(
        &self,
        client: &Client,
        id: ObjectId,
    ) -> Result<Vec<Raffle>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());
        collection
            .find(doc! {"_id": id.clone()}, None)
            .await?
            .try_collect()
            .await
    }

    pub async fn get_ticket_by_id(
        &self,
        client: &Client,
        id: ObjectId,
    ) -> Result<Vec<Ticket>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());
        collection
            .find(doc! {"_id": id}, None)
            .await?
            .try_collect()
            .await
    }
    //endregion

    //region === FIND SPECIAL ===
    pub async fn get_tickets_by_id_raffle(
        &self,
        client: &Client,
        id: ObjectId,
    ) -> Result<Vec<Ticket>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());
        collection
            .find(doc! {"raffle_id": id}, None)
            .await?
            .try_collect()
            .await
    }

    pub async fn get_spl_tx_in_ticket(
        &self,
        client: &Client,
        spl_tx_signature: &String,
    ) -> Result<Option<Ticket>, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());

        collection
            .find_one(doc! {"spl_tx_signature": spl_tx_signature}, None)
            .await
    }
    //endregion

    //region === UPDATE ===
    pub async fn update_raffle(
        &self,
        client: &Client,
        raffle: &mut Raffle,
    ) -> mongodb::error::Result<UpdateResult> {
        raffle.date_updated = chrono::Utc::now().timestamp();

        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());

        let r = raffle.clone();
        let doc = doc! {
                "$set":{
                "title": r.title,
                "description": r.description,
                "status": r.status,
                "ticket_amount": r.ticket_amount as i32,
                "ticket_price": r.ticket_price as f32,
                "ticket_token_name": r.ticket_token_name,
                "rule": r.rule,
                "date_updated": chrono::Utc::now().timestamp()
        }};
        collection.update_one(doc! {"_id": r.id}, doc, None).await
    }
    pub async fn update_ticket(
        &self,
        client: &Client,
        ticket: &Ticket,
    ) -> mongodb::error::Result<UpdateResult> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Ticket>(COLL_TICKET.as_ref());

        let t = ticket.clone();

        let doc = doc! {
                "$set":{
                "username": t.username
        }};
        collection.update_one(doc! {"_id": t.id}, doc, None).await
    }
    //endregion
}
