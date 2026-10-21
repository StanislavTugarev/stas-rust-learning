fn main() {
    let opt = Some(42);
    let x = opt.iter().next().unwrap();
    for value in opt.iter() {
        println!("{value}");
    }

    let opt = Some(42);
    for value in opt.into_iter() {
        println!("{value}");
    }

    let mut opt = Some(10);
    for value in opt.iter_mut() {
        *value += 10;
        println!("{value}");
    }

    let mut vec = vec![Some(1), Some(2), Some(3)];
    for item in vec.iter() {
        let value = item.as_ref().unwrap();
    }

    for item in vec.iter_mut() {
        let value = item.as_mut().unwrap();
    }

    for item in vec.into_iter() {
        match item {
            Some(inner) => println!("Dom some processing"),
            None => println!("Value is none"),
        }
    }
}
