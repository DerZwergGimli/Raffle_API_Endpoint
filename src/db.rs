use crate::{ObjectId, Raffle, Ticket};
use actix_web::body::None;
use futures::future::ok;
use log::*;
use mongodb::error::Error;
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use mongodb::{Client, Collection, Database};
use std::result;

use crate::config_loader::ConfigFile;
use futures::stream::{StreamExt, TryStreamExt};
use futures::TryFutureExt;
use mongodb::bson::{bson, doc};
use mongodb::options::UpdateModifications;

const COLL_RAFFLE: &str = "raffle";

enum TESTING {
    UpdateResult,
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
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());
        raffle.id = ObjectId::new();
        collection.insert_one(raffle, None).await
    }

    pub async fn insert_ticket(
        &self,
        client: &Client,
        ticket: &Ticket,
    ) -> Result<InsertOneResult, Error> {
        let collection = client
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_ticket.as_str());
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
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());
        collection.delete_one(doc! {"_id": raffle_id}, None).await
    }

    pub async fn remove_ticket(
        &self,
        client: &Client,
        ticket_id: ObjectId,
    ) -> Result<DeleteResult, Error> {
        let collection = client
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_ticket.as_str());
        collection.delete_one(doc! {"_id": ticket_id}, None).await
    }
    //endregion

    //region === FIND ALL ===
    pub async fn get_all_raffles(&self, client: &Client) -> Result<Vec<Raffle>, Error> {
        let collection = client
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());
        collection.find(None, None).await?.try_collect().await
    }

    pub async fn get_all_tickets(&self, client: &Client) -> Result<Vec<Ticket>, Error> {
        let collection = client
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_ticket.as_str());

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
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());
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
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_raffle.as_str());
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
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_raffle.as_str());
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
            .database(self.database_name.as_str())
            .collection::<Ticket>(self.collection_ticket.as_str());

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
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());

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
            .database(self.database_name.as_str())
            .collection::<Raffle>(self.collection_raffle.as_str());

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
