use rand::Rng;

// Result<Option<T>, E>
// Three-way Outcome -> Ok(Some(val))   for success value
//                   -> Ok(None)        for success but no value
//                   -> Err(e)          for failure

struct Product {
    id: u32,
    name: String,
}

enum DBerror {
    ConnectionFailed,
}

fn db_connection() -> Result<(), DBerror> {
    let mut rng = rand::thread_rng();
    if rng.gen_range(0.0..1.0) < 0.1 {
        Err(DBerror::ConnectionFailed)
    } else {
        Ok(())
    }
}

fn find_product(id: u32) -> Result<Option<Product>, DBerror> {
    db_connection()?;
    match id {
        0..100 => Ok(Some(Product {
            id,
            name: "Laptop".to_string(),
        })),
        _ => Ok(None),
    }
}

fn main() {}
