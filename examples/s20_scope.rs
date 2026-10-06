use std::{thread, vec};

fn main() {
    let mut vec = vec![1, 2, 3];
    // thread::spawn(move || println!("{:?}", vec));

    // println!("{:?}", vec);

    thread::scope(|some_scope| {
        let b = some_scope.spawn(|| {
            println!("Thread inside scope");
            println!("vec: {:?}", vec);
            1
        });

        let a = some_scope.spawn(|| {
            println!("Another thread inside scope");
            // we cant violate the borrowing rules
            // vec.push(4);
            println!("vec: {:?}", vec);
            1
        });
        // (a.join(), b.join())
    });

    println!("The scope finished");
    vec.push(5);
    println!("vec: {:?}", vec)
}
