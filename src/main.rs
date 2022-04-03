use std::env;
use std::fs::File;
use std::io::BufReader;
use actix_web::{App, Error, HttpServer, web};
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::middleware::HttpAuthentication;
use log::*;
use mongodb::{Client};
use mongodb::bson::oid::ObjectId;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use api::*;
use db::DatabaseRaffle;
use model::*;



mod api;
mod config_loader;
mod db;
mod model;
mod mongo_index;
mod solscan_api;
mod validator;

//use solana_sdk::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger Setup
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting...");

    let server_address = format!("{}:{}", env::var("SERVER_IP").unwrap(), env::var("SERVER_PORT").unwrap());

    //Server Setup
    let m_uri = env::var("MONGODB_URI").unwrap();
    let client = Client::with_uri_str(m_uri).await.expect("failed to connect");
    let db_interface = DatabaseRaffle::new();
    let config = load_certificate();
    info!(
        "Server available at: https:://{} ", server_address
    );

    HttpServer::new(move || {
        let middleware = HttpAuthentication::bearer(token_validator);
        App::new()
            .wrap(middleware)
            .app_data(web::Data::new(client.clone()))
            .app_data(web::Data::new(db_interface.clone()))
            .app_data(web::Data::new(config_loader::load_config_file().clone()))
            .service(
                web::scope("/api/v1")
                    // API-POST
                    .service(add_raffle)
                    .service(add_ticket)
                    // API-GET
                    .service(get_raffle)
                    .service(get_ticket)
                    // API-DELETE
                    .service(remove_raffle)
                    .service(remove_ticket)
                    // API-UPDATE
                    .service(update_raffle)
                    .service(update_ticket),
            )
    })
        /*.bind("localhost:8080")?*/
        .bind_rustls(
            server_address,
            config,
        )?
        .run()
        .await
}

async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {


    let tokens = vec![env::var("API_BEARER_TOKEN").unwrap()];

    let mut valid = false;
    for token in tokens {
        if token == credentials.token() {
            valid = true;
        }
    }

    if valid {
        Ok(req)
    } else {
        let config = req
            .app_data::<Config>().cloned()
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err(AuthenticationError::from(config).into())
    }
}

fn load_certificate() -> ServerConfig {
    //Cert Setup
    // load ssl keys
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("cert/key.pem").unwrap());
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }
    let config = config.with_single_cert(cert_chain, keys.remove(0)).unwrap();
    config
}
