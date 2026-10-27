struct UnsizedStruct {
    sized_field_1: i32,
    // only the last field of a struct may have a dynamically sized type
    unsized_field: [i32],
}

struct UnsizedStruct2<T: ?Sized> {
    sized_field: i32,
    unsized_field: T,
}

fn print_fn<T: std::fmt::Debug + Sized>(t: &T) {
    println!("{:?}", t)
}

fn main() {
    // we can't initialize this struct
    let vec = vec![3, 2, 1];
    let x = UnsizedStruct {
        sized_field_1: 3,
        unsized_field: vec.as_array(),
    };
    let x = UnsizedStruct2 {
        sized_field: 3,
        unsized_field: [3, 2, 1],
    };

    let x = "my_name";
    print_fn(&x);
}
