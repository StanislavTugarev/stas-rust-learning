trait SomeTrait {
    fn method(&self);
}

impl<T> SomeTrait for [T] {
    fn method(&self) {}
    // any &[T]
    // Vec<T>
    // [T; N]
}

fn main() {
    let slice: &[i32] = &[1];
    let vec = vec![1];
    let array = [1, 2, 3];

    array_slice_fn(slice);
    array_slice_fn(&vec); // deref coercion
    array_slice_fn(&array); // unsized coercion

    slice.method();
    vec.method();
    array.method();
}

fn array_slice_fn<T>(s: &[T]) {}
