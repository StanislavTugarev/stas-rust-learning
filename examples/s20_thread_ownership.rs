use std::thread;

fn main() {
    let x = "some string".to_string();

    let my_closure = || {
        let y = x;
        println!("{y}");
    };

    thread::spawn(my_closure);

    let x1 = "some".to_string();

    let my_closure = move || println!("{x1}");
    my_closure();
    my_closure();
    thread::spawn(my_closure);
}
