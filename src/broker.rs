use futures::stream::StreamExt;
use lapin::{message::Delivery, options::*, types::FieldTable, Connection, ConnectionProperties};
use tokio;

use crate::email;
use crate::tg;

#[tokio::main]
pub async fn start() {
    let broker_uri = std::env::var("BROKER_URI").unwrap();

    let conn = Connection::connect(&broker_uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let tg_channel = conn
        .create_channel()
        .await
        .expect("Failed to create tg channel");
    let email_channel = conn
        .create_channel()
        .await
        .expect("Failed to create email channel");

    tg_channel
        .queue_declare(
            "tg_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare tg queue");

    email_channel
        .queue_declare(
            "email_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare email queue");

    let tg_consumer_handle = {
        let mut tg_consumer = tg_channel
            .basic_consume(
                "tg_queue",
                "tg_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to start tg consumer");

        tokio::spawn(async move {
            while let Some(delivery_result) = tg_consumer.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        handle_tg_message(delivery).await;
                    }
                    Err(error) => eprintln!("Error receiving message from tg queue: {:?}", error),
                }
            }
        })
    };

    let email_consumer_handle = {
        let mut email_consumer = email_channel
            .basic_consume(
                "email_queue",
                "email_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to start email consumer");

        tokio::spawn(async move {
            while let Some(delivery_result) = email_consumer.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        handle_email_message(delivery).await;
                    }
                    Err(error) => {
                        eprintln!("Error receiving message from email queue: {:?}", error)
                    }
                }
            }
        })
    };

    println!("Hello");

    _ = tokio::join!(tg_consumer_handle, email_consumer_handle);
}

async fn handle_tg_message(delivery: Delivery) {
    let body = String::from_utf8_lossy(&delivery.data);

    tg::send(body.to_string()).await;

    delivery
        .ack(BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message from tg queue");
}

async fn handle_email_message(delivery: Delivery) {
    let body = String::from_utf8_lossy(&delivery.data);

    email::send(body.to_string()).await;

    delivery
        .ack(BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message from email queue");
}
