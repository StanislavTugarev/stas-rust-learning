use core::{error, fmt};
use std::{f32::consts::E, fmt::write, fs::File, io::Read, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("I/O Error:")]
    Io(#[from] std::io::Error),

    #[error("Parse Error:")]
    Parse(#[from] ParseIntError),
}

// impl fmt::Display for AppError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Result<(), fmt::Error>
//         match self {
//             AppError::Io(e) => write!(f, "I/O Error: {}", e),
//             AppError::Parse(e) => write!(f, "Parse Error: {}", e),
//         }
//     }
// }

// impl From<Error> for AppError {
//     fn from(value: Error) -> Self {
//         Self::Io(value)
//     }
// }

// impl From<ParseIntError> for AppError {
//     fn from(value: ParseIntError) -> Self {
//         Self::Parse(value)
//     }
// }

fn read_and_parse_number(file_path: String) -> Result<i32, AppError> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let number = contents.trim().parse::<i32>()?;
    Ok(number)
}

fn main() {
    let file = "number.txt".to_string();
    match read_and_parse_number(file) {
        Ok(number) => println!("File contains number: {}", number),
        Err(e) => {
            println!("Cannot process: {:?}", e);
            match e {
                AppError::Io(err) => std::process::exit(1),
                AppError::Parse(err) => std::process::exit(2),
            }
        }
    }
}
