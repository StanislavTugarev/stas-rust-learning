use std::ops::Deref;

// A -> B and B -> C then A -> C
struct Book {
    name: String,
    authors: String,
    public_year: u32,
}

impl Deref for Book {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let my_book = Book {
        name: "Rust Programming".to_string(),
        authors: "Nouman Azam".to_string(),
        public_year: 2020,
    };

    // Transitive Deref Coercion Chain:
    // &Book -> &String -> &str
    print_str(&my_book);

    // &Box<Book> -> &Book -> &String -> &str
    print_str(&Box::new(my_book));
}
