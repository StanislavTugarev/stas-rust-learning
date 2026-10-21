fn main() {
    let data = vec![Some(1), Some(2), Some(3)];
    let unwrapped = data
        .into_iter()
        .map(|x| x.unwrap() * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", unwrapped);

    let data = vec![Some(1), None, Some(3), None, Some(5)];
    let filtered = data
        .into_iter()
        .flatten()
        // .filter(|x| x.is_some())
        // .map(|x| x.unwrap())
        .collect::<Vec<i32>>();
    println!("Filtered data: {:?}", filtered);

    let mut data = vec![Some(1), None, Some(2), None, Some(3)];
    let updated_data = data
        .into_iter()
        .filter(|x| x.is_some())
        .collect::<Vec<Option<i32>>>();
    println!("Data with None removed {:?}", updated_data);
}
