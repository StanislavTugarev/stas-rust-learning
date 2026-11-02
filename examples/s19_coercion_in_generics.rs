struct A;

trait SomeTrait {}

impl SomeTrait for &A {}
impl SomeTrait for &mut A {}

fn main() {
    let mut a = A;
    generic(&a);
    generic(&mut a);
}

fn generic<T: SomeTrait>(x: T) {}
//fn generic_immutable_ref(x: &A) {}
//fn generic_mutable_ref(x: &mut A) {}
