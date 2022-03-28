use crate::config_loader::ConfigFile;

use crate::{validator, DatabaseRaffle, ObjectId};

use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer};
use log::{error, info};

use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};

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
            info!("{:?}", data);
            HttpResponse::Ok().body("ok")
        }
        Err(err) => {
            error!("{:?}", data);
            HttpResponse::InternalServerError().body(format!("{:?}", err))
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
    let tickets = 0;

    match validator::validate_ticket(&client, &db_interface, ticket.clone(), &conf).await {
        Ok(tickets) => {
            ticket.amount = tickets;
            let result = db_interface.insert_ticket(&client, &mut ticket).await;

            match result {
                Ok(_) => {
                    info!("{:?}", ticket);
                    HttpResponse::Ok().body("ok")
                }
                Err(err) => {
                    error!("{:?}", err);
                    HttpResponse::InternalServerError().body(err.to_string())
                }
            }
        }

        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err.to_string())),
    }
    //info!("Tickets_out={:?}", tickets);
    /*if tickets != 0 {
        ticket.amount = tickets;
        let result = db_interface.insert_ticket(&client, &mut ticket).await;

        match result {
            Ok(_) => {
                info!("{:?}", ticket);
                HttpResponse::Ok().body("ok")
            }
            Err(err) => {
                error!("{:?}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    } else {
        HttpResponse::Ok().body(format!("Invalid input: {:?} Tickets were added!", tickets))
    }*/
}
//endregion

//region === GET ===
#[get("/raffle/{id}")]
pub async fn get_raffle(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let oid = id.into_inner();
    let result = match oid.as_str() {
        "0" => db_interface.get_all_raffles(&client).await,
        _ => {
            let data = ObjectId::parse_str(oid.as_str()).unwrap();
            db_interface.get_raffle_by_id(&client, data).await
        }
    };
    match result {
        Ok(_) => {
            info!("{:?}", result);
            HttpResponse::Ok().json(result.unwrap())
        }
        Err(err) => {
            error!("{:?}", err);
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
    let oid = id.into_inner();
    let result = match oid.as_str() {
        "0" => db_interface.get_all_tickets(&client).await,
        _ => {
            let data = ObjectId::parse_str(oid.as_str()).unwrap();
            db_interface.get_ticket_by_id(&client, data).await
        }
    };
    match result {
        Ok(_) => {
            info!("{:?}", result);
            HttpResponse::Ok().json(result.unwrap())
        }
        Err(err) => {
            error!("{:?}", err);
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
        Ok(result) => {
            info!("Updated {:?}", data);
            HttpResponse::Ok().body(format!("{:?}", result.matched_count))
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
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
        Ok(result) => {
            info!("{:?}", data);
            HttpResponse::Ok().body(format!("{:?}", result.matched_count))
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
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
    let data = ObjectId::parse_str(id.into_inner()).unwrap();
    let result = db_interface.remove_raffle(&client, data).await;
    match result {
        Ok(_) => {
            info!("{:?}", data);
            HttpResponse::Ok().body("ok")
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

#[delete("/ticket/{id}")]
pub async fn remove_ticket(
    client: web::Data<Client>,
    db_interface: web::Data<DatabaseRaffle>,
    id: web::Path<String>,
) -> HttpResponse {
    let data = ObjectId::parse_str(id.into_inner()).unwrap();
    let result = db_interface.remove_ticket(&client, data).await;
    match result {
        Ok(_) => {
            info!("{:?}", data);
            HttpResponse::Ok().body("ok")
        }
        Err(err) => {
            error!("{:?}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}
//endregion
