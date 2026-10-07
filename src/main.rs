use std::time::Duration;
use tokio::time::sleep;

// #[tokio::main(flavor = "current_thread")]
#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("11 Task {i} printing, first time");
            printing(i).await;
            println!("13 Task {i}, printing, second time");
            printing(i).await;
            println!("15 Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        println!("20 Handle await");
        handle.await.unwrap();
    }
}

async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("27 Task {i}")
}

async fn my_fn() -> i32 {
    5
}
