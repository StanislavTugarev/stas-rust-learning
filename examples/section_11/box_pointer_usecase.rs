#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>), // we can use less heap space, using option instead of putting Nil in a box
}

struct Huge_Data;
struct Small_Data;
trait Storage {}

impl Storage for Huge_Data {}
impl Storage for Small_Data {}

fn main(){
    let list = List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))));
    println!("{:?}", list);

    let data_1 = Huge_Data;
    let data_2 = Box::new(Huge_Data);

    let data_3 = data_1; // the entire data will be copied, because all data in stack
    let data_4 = data_2; // only the box pointer will be copied

    let data_5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5]; // storage all types of data implementing Storage trait
}