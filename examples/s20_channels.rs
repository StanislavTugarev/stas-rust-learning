use std::{sync::mpsc, thread};
fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            // let mut i = "5".to_string();
            println!("Sending value: {i}");
            tx_clone.send(i).unwrap();
            // println!("Val is: {i}");
        });
    }

    drop(tx);

    // let received_val = rx.recv().unwrap();
    // println!("Received: {received_val}");

    // let received_val = rx.recv().unwrap();
    // println!("Received: {received_val}");

    for message in rx {
        println!("Received {message}");
    }
}
