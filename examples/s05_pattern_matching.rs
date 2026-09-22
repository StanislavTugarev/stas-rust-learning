// Value: the thing you are trying to match against
// Pattern: The shape of structure you are mathing

fn main() {
    // Match expression
    let x = 3;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }
    // Value: x
    // Pattern: 1, 2, 3, _

    // if let
    let x = 3;
    // if x == 5
    if let 5 = x {
        println!("Matched Five");
    }
    // Value: x
    // Pattern: 5

    if let x = 5 {
        // pattern always matches, let x =5
        println!("It always run");
        println!("x: inner {x}");
    }

    println!("x: outer {x}");

    // Binding pattern
    // Value: concrete value
    // Pattern: variable

    // while let
    let numbers = vec![1, 2, 2, 3, 2, 0];
    let mut i = 1;
    // the loop will execute only only if the curent value of numbers[i] = 2, if not - it'll skip
    // in this example, lopp will execute only 2 times
    while let 2 = numbers[i] {
        println!("Found a value 2 at index: {}", i);
        i += 1;
    }
    // Value: number[i]
    // Pattern: 2

    // let binding
    let (a, b) = (10, 20);
    // Value: (10, 20)
    // Pattern: (a, b)

    // function paramters
    let point = (5, 8);
    print_coords(point);
    // Value: (5, 8)
    // Pattern: (x, y)
    // Type: (i32, i32)
}

fn print_coords((x, y): (i32, i32)) {
    println!("x: {x}, y: {y}");
}
