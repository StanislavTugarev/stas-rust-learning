use std::{i32, sync::mpsc, thread, time::Duration};

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut x = "some value".to_string();
        println!("Sending value: {x}");
        // thread::sleep(Duration::from_secs(3));
        tx.send(x).unwrap();
    });

    let a = Point { x: 1, y: 1 };
    let b = || {
        let b = &a;
        println!("{:?}", b)
    };

    thread::spawn(b);

    // let recv_val = rx.recv().unwrap();
    // println!("I am Blocked");

    let mut recieved_status = false;
    while recieved_status != true {
        match rx.try_recv() {
            Ok(received_value) => {
                println!("Received value is: {received_value}");
                recieved_status = true;
            }
            Err(_) => println!("I am doing some other stuff"),
        }
    }
}
