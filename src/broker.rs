use futures::stream::StreamExt;
use lapin::{message::Delivery, options::*, types::FieldTable, Connection, ConnectionProperties};
use std::collections::HashSet;
use std::sync::Arc;
use tokio;
use tokio::sync::Mutex;

use crate::email;
use crate::tg;

struct MyStruct {
    tg_set: Arc<Mutex<HashSet<i64>>>,
    email_set: Arc<Mutex<HashSet<i64>>>,
}

#[tokio::main]
pub async fn start() {
    let broker_uri = std::env::var("BROKER_URI").unwrap();

    let conn = Connection::connect(&broker_uri, ConnectionProperties::default())
        .await
        .expect("Failed to connect to RabbitMQ");

    let channel1 = conn
        .create_channel()
        .await
        .expect("Failed to create channel 1");
    let channel2 = conn
        .create_channel()
        .await
        .expect("Failed to create channel 2");

    let _queue1 = channel1
        .queue_declare(
            "test_queue_1",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue 1");

    let _queue2 = channel2
        .queue_declare(
            "test_queue_2",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue 2");

    let shared_data = Arc::new(MyStruct {
        tg_set: Arc::new(Mutex::new(HashSet::new())),
        email_set: Arc::new(Mutex::new(HashSet::new())),
    });

    let consumer1_handle = {
        let shared_data = Arc::clone(&shared_data);
        let mut consumer1 = channel1
            .basic_consume(
                "test_queue_1",
                "my_consumer_1",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to start consumer 1");

        tokio::spawn(async move {
            while let Some(delivery_result) = consumer1.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        handle_tg_message(shared_data.clone(), delivery).await;
                    }
                    Err(error) => eprintln!("Error receiving message from queue 1: {:?}", error),
                }
            }
        })
    };

    let consumer2_handle = {
        let shared_data = Arc::clone(&shared_data);
        let mut consumer2 = channel2
            .basic_consume(
                "test_queue_2",
                "my_consumer_2",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failed to start consumer 2");

        tokio::spawn(async move {
            while let Some(delivery_result) = consumer2.next().await {
                match delivery_result {
                    Ok(delivery) => {
                        handle_email_message(shared_data.clone(), delivery).await;
                    }
                    Err(error) => eprintln!("Error receiving message from queue 2: {:?}", error),
                }
            }
        })
    };

    _ = tokio::join!(consumer1_handle, consumer2_handle);
}

async fn handle_tg_message(shared_data: Arc<MyStruct>, delivery: Delivery) {
    let body = String::from_utf8_lossy(&delivery.data);

    {
        let mut tg_set = shared_data.tg_set.lock().await;
        tg_set.insert(body.len() as i64);
    }

    _ = tg::send("body".to_string());

    delivery
        .ack(BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message from queue 1 (Telegram)");
}

async fn handle_email_message(shared_data: Arc<MyStruct>, delivery: Delivery) {
    let body = String::from_utf8_lossy(&delivery.data);

    {
        let mut email_set = shared_data.email_set.lock().await;
        email_set.insert(body.len() as i64);
    }

    email::send(body.to_string()).await;

    delivery
        .ack(BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message from queue 2 (Email)");
}
