use std::fmt::{Debug, Display};

use super::error::AocResult;

pub struct AocRunner;

impl AocRunner {
    pub fn run<T: Display + Debug>(result: AocResult<T>, context: &str) {
        if result.is_ok() {
            println!("Success! {}{}", context, result.unwrap());
        } else {
            println!("Failure :( Error: {}", result.unwrap_err());
        }
    }
}
