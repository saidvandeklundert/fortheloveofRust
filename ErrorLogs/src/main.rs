use std::fs;
use std::io::Error; // Struct that can be used to create custom errors for IO related 'things'

fn main() {
    examples();
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text); // text is wrapped inside Ok
    let mut result : Vec<String> = vec![];
    match fs::read_to_string("logs.txt"){
        Ok(text_that_was_read)=> {
            println!("lenght of text that was read: {}", text_that_was_read.len());
            result = extract_errors(text_that_was_read.as_str());            
        } // text_that_was_read goes out of scope and will be dropped
        Err(error)=> {
            println!("failed to read the file due to the following error: {}", error)
        }
    }

    println!("{:#?}", result);
    match fs::write("errors.txt", result.join("\n")){
        Ok(()) => println!("writing to file went OK!"),
        Err(error)=> println!("ran into an error when writing to file: {}", error)
        }
    /// instead of dealing with the Result returns, you can also turn to the methods attached to Result:
    /// unwrap: for quick debugging
    /// expect: for when you cannot recover from an error
    /// unwrap_or: to put in place a default value in case something went wrong
    let expected_text = fs::read_to_string("logs.txt").expect("file 'logs.txt' MUST exist");
    println!("{:#?}", expected_text);
    let result_from_expected_text = extract_errors(expected_text.as_str());  
    fs::write("errors.txt", result_from_expected_text.join("\n")).expect("failed to write to 'errors.txt'");
}

/// Important to realize here is the fact that the Vector of &str
/// has implications on how long the String has to live for the
/// reference to remain valid!
fn extract_errors(text:&str)-> Vec<String>{
    let mut result = vec![];
    let split_text = text.split("\n");
    for line in split_text{
        if line.starts_with("ERROR"){
            result.push(line.to_string())
        }
    }
    result

}

fn examples(){

    match divide(6.0, 0.0){
        Err(error) => println!("Err: {:#?}", error),
        Ok(result_of_division) => println!("Ok: {:#?}", result_of_division),        
    }
    match divide(6.0, 2.0){
        Err(error) => println!("Err: {:#?}", error),
        Ok(result_of_division) => println!("Ok: {:#?}", result_of_division),        
    }    


    match validate_email(String::from("some@email.com")){
        Err(error) => println!("Err: {:#?}", error),
        Ok(..) => println!("email is valid"), // .. indicates we will not be using the value from the Ok
    }    

    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives"),
    ];
    println!("{:#?}", validate_ingredients(&ingredients));    
}

/// Example where a Result is returned.
/// 
/// In case we cannot recover or it is completely
/// unexpected, we would turn to panic.
/// 
/// For all else, use the Result enum
fn divide(a: f64, b: f64) -> Result<f64,Error> {
    if b == 0.0{
        return Err(Error::other("zero division error"));
    }
    Ok(a/b)
}


/// If a function does not return anything, the
/// convention is to return a Result where the Ok
/// is filled with an empty tuple.
fn validate_email(email:String)->Result<(), Error>{
    if email.contains("@") {
        // succes case:
        Ok(())
    } else {
        return Err(Error::other("emails must contain a '@'"));
    }
}



fn validate_ingredients(ingredients: &Vec<String>)->Result<(),Error> {
    if ingredients.len() > 3 {
        return Err(Error::other("too many ingredients"))
    } else {
        Ok(())
    }
}