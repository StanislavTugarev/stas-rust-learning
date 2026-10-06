#[tokio::main]
async fn main() {
    let x = printing();
    println!("The future has not been polled yet");

    // drop(x);
    x.await;
}

async fn printing() {
    println!("I am async function")
}
