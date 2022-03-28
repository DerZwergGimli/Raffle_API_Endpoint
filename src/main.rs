mod api;
mod config_loader;
mod db;
mod model;
mod mongo_index;
mod solscan_api;
mod validator;

use actix_files::Files;
use actix_web::dev::ServiceRequest;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use api::*;
use db::DatabaseRaffle;
use log::*;
use log::{error, log};
use model::*;
use mongo_index::*;
use mongodb::bson::oid::ObjectId;
use mongodb::{bson::doc, options::IndexOptions, Client, Collection, IndexModel};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::*;
use solana_sdk::signature::Signature;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use validator::*;

//use solana_sdk::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logger Setup
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting...");

    let url = "http://localhost:8899".to_string();
    let client = RpcClient::new(url);

    //let sig_s = String::from(
    //    "3xrq4cysrDDZTNwXhVG3Cz4rQGxaRwuy5sa5yFEHzhWzTBdUYGZ1fQusv9nd2Qxez1Rr2jEQUTiT8zhoXsDHLbht",
    //);
    //let sig = Signature::from_str(&*sig_s).unwrap();
    //let stat = solana_client.get_signature_status(&sig).await.unwrap();
    //println!("{:?}", stat);
    //TODO
    // https://stackoverflow.com/questions/71107464/get-solana-transaction-status-using-rust

    //Server Setup
    let uri = config_loader::load_config_file().mongodb_uri;
    let client = Client::with_uri_str(uri).await.expect("failed to connect");
    let db_interface = DatabaseRaffle::new(config_loader::load_config_file().clone());
    let config = load_certificate();
    info!(
        "Server available at: https://{:?}:{:?} ",
        config_loader::load_config_file().server_ip,
        config_loader::load_config_file().server_port,
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
        (
            config_loader::load_config_file().server_ip,
            config_loader::load_config_file().server_port,
        ),
        config,
    )?
    .run()
    .await
}

async fn token_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let tokens = config_loader::load_config_file().access_tokens;

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
            .app_data::<Config>()
            .map(|data| data.clone())
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
