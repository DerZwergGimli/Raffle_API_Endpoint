use crate::config_loader::ConfigFile;
use crate::solscan_api::get_solana_tx;
use crate::{solscan_api, validator, DatabaseRaffle, ObjectId};
use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer};
use log::{error, info};
use mongodb::error::Error;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use std::str::FromStr;

use super::db;
use super::model::*;

//region === POST ===
#[post("/raffle")]
pub async fn add_raffle(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    form: web::Json<Raffle>,
) -> HttpResponse {
    let mut data = form.into_inner();
    let result = db_interface.insert_raffle(&client, &mut data).await;
    match result {
        Ok(_) => {
            info!("Added to DB: {:?}", data);
            HttpResponse::Ok().body("raffle added")
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to add to DB: {:?}", data);
            error!("Error: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Unable to add raffle"))
        }
    }
}

#[post("/ticket")]
pub async fn add_ticket(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    conf: web::Data<ConfigFile>,
    form: web::Json<Ticket>,
) -> HttpResponse {
    let mut ticket = form.into_inner();
    let tickets = validator::validate_ticket(&client, &db_interface, ticket.clone(), &conf).await;

    if tickets != 0 {
        ticket.amount = tickets;
        let result = db_interface.insert_ticket(&client, &ticket).await;

        match result {
            Ok(_) => {
                info!("Added to DB: {:?}", ticket);
                HttpResponse::Ok().body(format!("{:?} tickets added", tickets))
            }
            //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            Err(err) => {
                error!("Unable to add to DB: {:?}", ticket);
                HttpResponse::InternalServerError().body(format!("Unable to add ticket"))
            }
        }
    } else {
        HttpResponse::Ok().body(format!("Invalid input: {:?} Tickets were added!", tickets))
    }
}
//endregion

//region === GET ===
#[get("/raffle/{id}")]
pub async fn get_raffle(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let mut oid = id.into_inner();
    let mut result = match oid.as_str() {
        "0" => db_interface.get_all_raffles(&client).await,
        _ => {
            let mut data = ObjectId::parse_str(oid.as_str()).unwrap();
            db_interface.get_raffle_by_id(&client, data).await
        }
    };

    match result {
        Ok(_) => {
            info!("Found in DB: {:?}", result);
            HttpResponse::Ok().json(result.unwrap())
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to find id {:?}", oid);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[get("/ticket/{id}")]
pub async fn get_ticket(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let mut oid = id.into_inner();
    let mut result = match oid.as_str() {
        "0" => db_interface.get_all_tickets(&client).await,
        _ => {
            let mut data = ObjectId::parse_str(oid.as_str()).unwrap();
            db_interface.get_ticket_by_id(&client, data).await
        }
    };

    match result {
        Ok(_) => {
            info!("Found in DB: {:?}", result);
            HttpResponse::Ok().json(result.unwrap())
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to find id {:?}", oid);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
//endregion

//region == UPDATE ==
#[patch("/raffle/{id}")]
pub async fn update_raffle(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
    form: web::Json<Raffle>,
) -> HttpResponse {
    let mut data = form.into_inner();
    data.id = ObjectId::parse_str(id.into_inner().to_string()).unwrap();
    let result = db_interface.update_raffle(&client, &mut data).await;
    match result {
        Ok(_) => {
            info!("Updated to DB: {:?}", data);
            HttpResponse::Ok().body("raffle updated")
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to update to DB: {:?}", data);
            HttpResponse::InternalServerError().body(format!("Unable to update raffle"))
        }
    }
}

#[patch("/ticket/{id}")]
pub async fn update_ticket(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
    form: web::Json<Ticket>,
) -> HttpResponse {
    let mut data = form.into_inner();
    data.id = ObjectId::parse_str(id.into_inner().to_string()).unwrap();
    let result = db_interface.update_ticket(&client, &mut data).await;
    match result {
        Ok(_) => {
            info!("Updated to DB: {:?}", data);
            HttpResponse::Ok().body("ticket updated")
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to update to DB: {:?}", data);
            HttpResponse::InternalServerError().body(format!("Unable to update ticket"))
        }
    }
}
//endregion

//region === DELETE ===
#[delete("/raffle/{id}")]
pub async fn remove_raffle(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let mut data = ObjectId::parse_str(id.into_inner()).unwrap();
    let result = db_interface.remove_raffle(&client, data).await;
    match result {
        Ok(_) => {
            info!("Removed from DB: {:?}", data);
            HttpResponse::Ok().body("raffle removed")
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to remove from DB: {:?}", data);
            HttpResponse::InternalServerError().body(format!("Unable to remove raffle"))
        }
    }
}

#[delete("/ticket/{id}")]
pub async fn remove_ticket(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let mut data = ObjectId::parse_str(id.into_inner()).unwrap();
    let result = db_interface.remove_ticket(&client, data).await;
    match result {
        Ok(_) => {
            info!("Removed from DB: {:?}", data);
            HttpResponse::Ok().body("ticket removed")
        }
        //Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        Err(err) => {
            error!("Unable to remove from DB: {:?}", data);
            HttpResponse::InternalServerError().body(format!("Unable to remove ticket"))
        }
    }
}
//endregion
