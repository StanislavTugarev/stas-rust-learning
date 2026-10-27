#![feature(never_type)]

enum NeverType {}

fn main() {
    unrecoverable_state();
    let x = unrecoverable_state();
    let x: !;

    let x = match "123".parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    let x = return;
    let x: String = return;

    let counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    };

    let x: NeverType;
}

fn unrecoverable_state() -> ! {
    panic!("This function will never return normally with something valid");
}

fn function() -> Result<i32, String> {
    Ok(1)
}

fn function_1() -> Result<i32, !> {
    Ok(1)
}

fn function_2() -> Result<!, i32> {
    Ok((panic!()))
}
