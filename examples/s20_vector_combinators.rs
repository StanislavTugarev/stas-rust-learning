fn main() {
    let csv_data = vec!["name, age", "Alice, 30", "Bob, 25"];
    let data_without_header = csv_data.into_iter().skip(1).collect::<Vec<&str>>();
    println!("First row removed: {:?}", data_without_header);

    let data = vec![
        "row1".to_string(),
        "row2".to_string(),
        "row3".to_string(),
        "row4".to_string(),
    ];
    let preview = data.iter().take(2).collect::<Vec<&String>>();
    println!("Data preview: {:?}", preview);

    let code = vec!["fn main() {", "println!(\"Hello\")", "}"];
    let code_with_line_numbers = code.into_iter().enumerate().collect::<Vec<(usize, &str)>>();
    println!("Code with line number {:?}", code_with_line_numbers);

    let nums = vec![1, 2, 3, 4];
    let closure = |acc: i32, &x: &i32| acc + x * x;
    let sum_of_square = nums.iter().fold(0, |acc, &x| acc + x * x);
    println!("Sum of squares of 1, 2, 3, 4 is: {sum_of_square}");

    let nums = vec![1, 2, 3, 4];
    let result = nums.into_iter().reduce(|a, b| a.max(b));
    println!("{}", result.unwrap());
}
