use tokio::time::{Duration, sleep};
use tracing::{debug, error, field::debug, info, instrument, subscriber};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Setting tracing subscriber failed");
    let task_1 = tokio::spawn(process_request(1, 42));
    let task_2 = tokio::spawn(process_request(2, 43));

    let _ = tokio::join!(task_1, task_2);
}

#[instrument]
async fn process_request(user_id: u32, order_id: u32) {
    authenticate(user_id).await;

    if validate_order(order_id).await.is_err() {
        error!(%order_id, %user_id, "Validation failed");
        return;
    }

    match save_to_db(order_id).await {
        Ok(_) => info!(%order_id, "Order saved successfully"),
        Err(e) => error!(%order_id, error = %e, "DB save failed"),
    }
}

#[instrument]
async fn authenticate(user_id: u32) {
    debug!(%user_id, "Authenticating user");
    sleep(Duration::from_millis(50)).await;
    info!(%user_id, "Authenticating successful")
}

#[instrument]
async fn validate_order(order_id: u32) -> Result<(), &'static str> {
    debug!(%order_id, "Validating order");
    sleep(Duration::from_millis(80)).await;

    if order_id % 2 == 0 {
        Ok(())
    } else {
        Err("Invalid order quantity")
    }
}

#[instrument]
async fn save_to_db(order_id: u32) -> Result<(), &'static str> {
    debug!(%order_id, "Saving order to database");
    sleep(Duration::from_millis(50));
    Ok(())
}
