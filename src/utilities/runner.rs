use std::fmt::{Debug, Display};

use super::error::AocResult;

pub struct AocRunner;

impl AocRunner {
    pub fn run<T: Display + Debug>(result: AocResult<T>, context: &str) {
        match result {
            Ok(value) => println!("Success! {}{}", context, value),
            Err(err) => println!("Failure :( Error: {}", err)
        };
    }
}
