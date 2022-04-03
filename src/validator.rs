use std::env;

use log::info;
use mongodb::bson::oid::ObjectId;
use mongodb::Client;
use rust_decimal::prelude::ToPrimitive;
use snafu::{prelude::*, Whatever};

use crate::{DatabaseRaffle, solscan_api, Ticket};
use crate::solscan_api::SolanaTX;

pub async fn validate_ticket(
    client: &Client,
    db_interface: &DatabaseRaffle,
    ticket: Ticket,
) -> Result<u16, Whatever> {
    let tx = solscan_api::get_solana_tx(ticket.spl_tx_signature.clone()).await;

    info!("username={}", ticket.username);

    match tx {
        Ok(tx) => {
            info!("{:?}", tx);
            // Validate Ticket
            // Check if raffle_id is valid
            if env::var("CHECK_TOKEN_SYMBOL").unwrap_or_default().parse::<bool>().unwrap_or(false) && !check_token(client, db_interface, ticket.raffle_id, &tx).await {
                whatever!("Wrong token send in TX")
            };

            if env::var("CHECK_TX_STATUS").unwrap_or_default().parse::<bool>().unwrap_or(false) && !tx.status.contains("Success"){
                whatever!("SPL TX status not valid")
            };

            if env::var("CHECK_RAFFLE_EXISTS").unwrap_or_default().parse::<bool>().unwrap_or(false) && !check_if_raffle_exists(client, db_interface, ticket.raffle_id).await {
                whatever!("Raffle does not exist")
            };
            // Check if raffle is running
            if env::var("CHECK_RAFFLE_RUNNING").unwrap_or_default().parse::<bool>().unwrap_or(false) && !check_if_raffle_is_running(client, db_interface, ticket.raffle_id).await {
                whatever!("Raffle is not running")
            };

            // Check if date_time is valid
            if env::var("CHECK_RAFFLE_TIME").unwrap_or_default().parse::<bool>().unwrap_or(false) && !check_if_past_raffle_create(client, db_interface, ticket.raffle_id, &tx).await {
                whatever!("DateTime invalid")
            };

            // Check if tx_destination is valid
            if env::var("CHECK_RAFFLE_DESTINATION").unwrap_or_default().parse::<bool>().unwrap_or(false) && !check_if_tx_destination_valid(&tx).await {
                whatever!("Destination invalid")
            };

            // Check if spl_tx_signature is used
            if env::var("CHECK_RAFFLE_USED_SIGNATURE").unwrap_or_default().parse::<bool>().unwrap_or(false) && check_if_spl_signature_is_used(client, db_interface, &ticket.spl_tx_signature).await {
                whatever!("SPL Signature already used")
            };


            // Calculate valid ticket amount
            let tickets =
                calculate_ticket_amount(client, db_interface, ticket.raffle_id, tx.amount).await;
            if tickets == 0 {
                whatever!("Ticket amount would be 0")
            };
            Ok(tickets)
        }
        Err(e) => whatever!("API-Error {}", e),
    }
}

async fn check_if_raffle_exists(
    client: &Client,
    db_interface: &DatabaseRaffle,
    oid: ObjectId,
) -> bool {
    let raffle = db_interface.get_raffle_by_id(client, oid).await.unwrap();
    !raffle.is_empty()
}

async fn check_if_raffle_is_running(
    client: &Client,
    db_interface: &DatabaseRaffle,
    oid: ObjectId,
) -> bool {
    let raffle = db_interface.get_raffle_by_id(client, oid).await.unwrap();
    raffle[0].status.contains("running")
}

async fn check_if_past_raffle_create(client: &Client,
                                     db_interface: &DatabaseRaffle,
                                     oid: ObjectId,
                                     tx: &SolanaTX) -> bool {
    let raffle = db_interface.get_raffle_by_id(client, oid).await.unwrap();
    tx.block_time > raffle[0].date_created
}


async fn check_token(client: &Client,
                     db_interface: &DatabaseRaffle,
                     oid: ObjectId,
                     tx: &SolanaTX) -> bool {
    let raffle = db_interface.get_raffle_by_id(client,oid).await.unwrap();

    raffle[0].ticket_token_name.contains(&tx.token_symbol)
}


async fn check_if_tx_destination_valid(tx: &SolanaTX) -> bool {
    tx.destination_owner.contains(&env::var("SOL_WALLET").unwrap().to_string())
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
        Some(_result) => true,
        None => false,
    }
}

async fn calculate_ticket_amount(
    client: &Client,
    db_interface: &DatabaseRaffle,
    raffle_id: ObjectId,
    usdc_amount: f32,
) -> u16 {
    let mut raffle = db_interface
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
            raffle[0].status = "closed".to_string();
            db_interface.update_raffle(client, &mut raffle[0]);
            input_value_ticket as u16
        } else {
            tickets_left as u16
        }
    } else {
        0.0 as u16
    }
}
