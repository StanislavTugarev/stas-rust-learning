fn main() {
    let inputs = vec![" 23 ", "NaN", "42", "abc", "35"];

    let avg = inputs
        .into_iter()
        .map(str::trim)
        .filter(|s| !s.is_empty() && s.chars().all(char::is_numeric))
        .map(|s| s.parse::<u32>().unwrap())
        .fold((0, 0), |(sum, count), x| (sum + x, count + 1));

    let average = avg.0 as f64 / avg.1 as f64;
    println!("Average age: {:.2}", average);

    let inputs = vec![" 23 ", "NaN", "42", "abc", "35"];

    let stage1: Vec<&str> = inputs.into_iter().collect();
    println!("Stage 1 (raw): {:?}", stage1);

    let stage2: Vec<&str> = stage1.into_iter().map(str::trim).collect();
    println!("Stage 2 (trimmed): {:?}", stage2);

    let stage3: Vec<&str> = stage2
        .into_iter()
        .filter(|s| !s.is_empty() && s.chars().all(char::is_numeric))
        .collect();
    println!("Stage 3 (filtered value): {:?}", stage3);

    let stage4: Vec<u32> = stage3
        .into_iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    println!("Stage 4 9parsed ints: {:?}", stage4);

    let stage5 = stage4
        .into_iter()
        .fold((0, 0), |(sum, count), x| (sum + x, count + 1));

    let average = stage5.0 as f64 / stage5.1 as f64;
    println!("Average age: {:.2}", average);

    let inputs = vec![" 23 ", "NaN", "42", "abc", "35"];
    let avg = inputs
        .into_iter()
        .inspect(|s| println!("[raw] {:?}", s))
        .map(str::trim)
        .inspect(|s| println!("[trimmed] {:?}", s))
        .filter(|s| !s.is_empty() && s.chars().all(char::is_numeric))
        .inspect(|s| println!("[valid] {:?}", s))
        .map(|s| s.parse::<u32>().unwrap())
        .inspect(|s| println!("[parsing] {:?}", s))
        .fold((0, 0), |(sum, count), x| (sum + x, count + 1));
}
