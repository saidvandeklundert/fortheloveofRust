use anyhow::{Result, anyhow};

#[derive(Debug)]
enum MyErrors {
    ConfigurationParsingError,
    ConfigurationGenerationError,
    ConfigurationRetrievalError,
}

impl std::fmt::Display for MyErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigurationParsingError => write!(f, "ConfigurationParsingError"),
            Self::ConfigurationGenerationError => write!(f, "ConfigurationGenerationError"),
            Self::ConfigurationRetrievalError => write!(f, "ConfigurationRetrievalError"),
        }
    }
}
impl std::error::Error for MyErrors {}

fn function_that_might_fail(fail:bool) -> Result<String> {
    if fail == true{
        Err(anyhow!(MyErrors::ConfigurationParsingError))
    } else{
        Ok(String::from("got a normal return"))
    }
}
fn main() {
    println!("Hello, world!");
    let result_1 = function_that_might_fail(true);
    let result_2 = function_that_might_fail(false);
    println!("result_1: {:?}", result_1);
    println!("result_2: {:?}", result_2);
    
}
