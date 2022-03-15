use crate::config_loader::ConfigFile;
use crate::{ObjectId, Raffle, Ticket};
use actix_web::body::None;
use futures::future::ok;
use futures::stream::{StreamExt, TryStreamExt};
use futures::TryFutureExt;
use log::*;
use mongodb::bson::{bson, doc};
use mongodb::error::Error;
use mongodb::options::UpdateModifications;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Client, Collection, Database};

use lazy_static::lazy_static;
use std::{env, result};

lazy_static! {
    static ref DB_NAME: String = env::var("DB_NAME").unwrap_or("DB_Raffle".to_string());
    static ref COLL_RAFFLE: String = env::var("COLL_RAFFLE").unwrap_or("Raffle".to_string());
    static ref COLL_TICKET: String = env::var("COLL_TICKET").unwrap_or("Ticket".to_string());
}

#[derive(Clone)]
pub struct DatabaseRaffle {
    pub(crate) database_name: String,
    pub(crate) collection_raffle: String,
    pub(crate) collection_ticket: String,
}

impl DatabaseRaffle {
    pub fn new(conf: ConfigFile) -> Self {
        Self {
            database_name: conf.database_name,
            collection_raffle: conf.collection_raffle,
            collection_ticket: conf.collection_ticket,
        }
    }

    //region === INSERT ===
    pub async fn insert_raffle(
        &self,
        client: &Client,
        raffle: &mut Raffle,
    ) -> Result<InsertOneResult, Error> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());
        raffle.id = ObjectId::new();
        collection.insert_one(raffle, None).await
    }

    pub async fn insert_ticket(
        &self,
        client: &Client,
        ticket: &Ticket,
    ) -> Result<InsertOneResult, Error> {
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
        raffle: &Raffle,
    ) -> mongodb::error::Result<UpdateResult> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());

        let mut r = raffle.clone();

        collection
            .update_one(
                doc! {"_id": r.id},
                doc! {"$set":{
                    "description":r.description,
                }},
                None,
            )
            .await
    }
    pub async fn update_ticket(
        &self,
        client: &Client,
        ticket: &Ticket,
    ) -> mongodb::error::Result<UpdateResult> {
        let collection = client
            .database(DB_NAME.as_ref())
            .collection::<Raffle>(COLL_RAFFLE.as_ref());

        let mut t = ticket.clone();

        collection
            .update_one(
                doc! {"_id": t.id},
                doc! {"$set":{
                    "description": t.amount.to_string(),
                }},
                None,
            )
            .await
    }
    //endregion
}
