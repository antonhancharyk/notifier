use futures::stream::StreamExt;
use lapin::message::Delivery;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use tokio;

//     // let my_env_var = env::var("MY_ENV_VAR").expect("MY_ENV_VAR not found");

#[tokio::main]
pub async fn start() {
    let conn = Connection::connect(
        "amqp://194.146.38.167:5672",
        ConnectionProperties::default(),
    )
    .await
    .expect("Failed to connect to RabbitMQ");

    println!("Connected to RabbitMQ");

    let channel = conn
        .create_channel()
        .await
        .expect("Failed to create channel");

    let _queue = channel
        .queue_declare(
            "test_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to declare queue");

    let mut consumer = channel
        .basic_consume(
            "test_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to start consumer");

    println!("Waiting for messages...");

    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                handle_message(channel.clone(), delivery).await;
            }
            Err(error) => eprintln!("Error receiving message: {:?}", error),
        }
    }
}

async fn handle_message(channel: lapin::Channel, delivery: Delivery) {
    let body = String::from_utf8_lossy(&delivery.data);

    println!("Received message: {}", body);

    channel
        .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
        .await
        .expect("Failed to acknowledge message");
}
