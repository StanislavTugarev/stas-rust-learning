// use negative_impl::negative_impl;

// #[negative_impl]
// impl !Send for ABC {}

// #[negative_impl]
// impl !Sync for ABC {}

use std::{marker::PhantomData, rc::Rc};

struct ABC {
    ensuring_no_send_sync: PhantomData<Rc<()>>,
}

fn main() {
    println!("{}", size_of::<ABC>());
    let x = ABC {
        ensuring_no_send_sync: PhantomData,
    };
}
