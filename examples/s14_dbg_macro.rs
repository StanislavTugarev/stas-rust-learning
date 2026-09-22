fn main() {
    // dbg! prints
    // 1. File name
    // 2. Line number
    // 3. The expression is written
    // 4. The evaluated value

    let scores = vec![72, 85, 90, 67, 95, 88];
    let mut max_score = 0;
    for score in scores {
        // dbg!(score);
        if score > max_score {
            max_score = score;
            dbg!(max_score, score);
        }
    }

    let x = 5;
    let y = dbg!(x + 4);

    let x = "4".to_string();
    // debug takes ownership
    dbg!(x);
    // println!("{x}");
}
