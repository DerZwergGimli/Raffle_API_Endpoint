use super::model::Raffle;
use crate::config_loader::ConfigFile;
use crate::solscan_api::SolanaTX;
use crate::{solscan_api, DatabaseRaffle, Ticket};
use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::{Client, Collection, Database};
use reqwest::StatusCode;
use rust_decimal::prelude::ToPrimitive;
use std::env;

pub enum Error {
    NotFound,
}

pub async fn validate_ticket(
    client: &Client,
    db_interface: &DatabaseRaffle,
    ticket: Ticket,
    config: &ConfigFile,
) -> Result<u16, StatusCode> {
    let tx = solscan_api::get_solana_tx(ticket.spl_tx_signature.clone()).await;

    info!("username={}", ticket.username);

    match tx {
        Ok(tx) => {
            info!("{:?}", tx);
            // Validate Ticket
            // 1. check if raffle_id is valid
            let test1 = check_if_raffle_exists(client, db_interface, ticket.raffle_id).await;

            // 2. check if tx_destination is valid
            let test2 =
                check_if_tx_destination_valid(&tx, &config.destination_account_address).await;
            // 3. check if spl_tx_signature is used
            let test3 =
                check_if_spl_signature_is_used(client, db_interface, &ticket.spl_tx_signature)
                    .await;
            // 4 calculate valid ticket amount
            // 4.1 calculate available ticket amount
            let tickets =
                calculate_ticket_amount(client, db_interface, ticket.raffle_id, tx.amount).await;

            info!("check_if_raffle_exists={}", test1);
            info!("check_if_tx_destination_valid={}", test2);
            info!("check_if_spl_signature_is_used={}", test3);

            match test1 && test2 && !test3 {
                true => Ok(tickets),
                false => Ok(0),
            }
        }
        Err(e) => Err(e),
    }
}

async fn check_if_raffle_exists(
    client: &Client,
    db_interface: &DatabaseRaffle,
    oid: ObjectId,
) -> bool {
    let raffle = db_interface.get_raffle_by_id(client, oid).await.unwrap();
    if !raffle.is_empty() {
        true
    } else {
        false
    }
}

async fn check_if_tx_destination_valid(tx: &SolanaTX, tx_destination: &String) -> bool {
    tx.destination_owner.contains(tx_destination)
}

async fn check_if_spl_signature_is_used(
    client: &Client,
    db_interface: &DatabaseRaffle,
    spl_signature: &String,
) -> bool {
    let result = db_interface
        .get_spl_tx_in_ticket(client, spl_signature)
        .await
        .unwrap();
    match result {
        Some(result) => true,
        None => false,
    }
}

async fn calculate_ticket_amount(
    client: &Client,
    db_interface: &DatabaseRaffle,
    raffle_id: ObjectId,
    usdc_amount: f32,
) -> u16 {
    let raffle = db_interface
        .get_raffle_by_id(client, raffle_id)
        .await
        .unwrap();

    let tickets = db_interface
        .get_tickets_by_id_raffle(client, raffle_id.clone())
        .await
        .unwrap();

    let mut sold_tickets = 0;
    for ticket in tickets {
        sold_tickets += ticket.amount
    }

    info!("input_usdc_amount={:?}", usdc_amount);
    info!("total_tickets={:?}", raffle[0].ticket_amount);
    info!("sold_tickets={:?}", sold_tickets);
    info!("ticket_price={:?}", raffle[0].ticket_price);

    let input_value_ticket = usdc_amount / raffle[0].ticket_price;
    let tickets_left = raffle[0].ticket_amount - sold_tickets;

    info!("input_value_ticket={:?}", input_value_ticket);
    info!("tickets_left={:?}", tickets_left);

    if tickets_left > 0 {
        if input_value_ticket <= tickets_left.to_f32().unwrap() {
            input_value_ticket as u16
        } else {
            0.0 as u16
        }
    } else {
        0.0 as u16
    }
}
