// two types of output
// - Normal Output - stdout
// - Error Output - stderr

fn main() {
    // stream output into a file
    // cargo run > stdoutput.txt
    println!("Program Output");
    let x: Option<i32> = None;
    // save an error log into a file
    // cargo run 2> error_log.txt
    // x.unwrap();

    // write to the error stream
    eprint!("Error 1: Error Message");
    eprintln!("Error 2: Error Message");

    // dbg also writes in an error stream
    dbg!(x);
}
