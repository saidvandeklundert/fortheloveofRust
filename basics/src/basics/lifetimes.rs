///
/// Lifetimes:
/// - When a variable goes out of scope, the value owned by it is _dropped_
/// - Values cannot be dropped if there are still active references to it
/// - References to a value cannot outlive the value they refer to
/// 

/// This function fails with the following message:
/// 9 | fn next_langauage(languages:&[String],current:&str)->&str{
///   |                             ---------         ----   ^ expected named lifetime parameter
///fn next_language_failing(languages:&[String],current:&str)->&str{
///    let mut found = false;
///    for language in languages.iter(){
///        if found{
///            return language
///        }
///        if language == current{
///            found = true;
///        }
///    }
///    languages.last().unwrap()
///}


/// If you have a function that takes in two or more references AND that also returns a
/// reference, Rust will make a HUGE assumption. 
/// 
/// That assumption is that
/// the return reference will point at data referred to by one of the arguments.
/// 
/// This is a problem in the below function because what the compiler does NOT know is
/// what variable the return value will reference. Will it be current or languages???
/// 
/// In case it would be referring to current, there would be a problem since that reference
/// goes out of scope at the end of the function.
/// 
/// We can use the lifetime annotation to help the compiler understand what &str will point
/// to when it is returned from the function. When we tell the compiler the &str points to
/// languages, all is well since that value lives on after the function.
/// In the below program, that is not the case for 'current'.
/// 
/// So we add the lifetime annotation, 'a', to both the 'languages' argument as well as the return value.
/// Now the compiler understands everything and we can compile the program.
fn next_language<'a>(languages:&'a [String],current:&str)->&'a str{
    let mut found = false;
    for language in languages.iter(){
        if found{
            return language
        }
        if language == current{
            found = true;
        }
    }
    languages.last().unwrap()
}

/// This is a complicated case for lifetime annotations. 
/// The reason is you cannot know whether or not the function will return 
/// lang_a or lang_b because it depends on the lenght of the input arguments that
/// are not known untill runtime.
/// 
/// The solution is to mark everything with the same lifetime annotation:
fn longest_language<'a>(lang_a: &'a str, lang_b: &'a str) -> &'a str{
    if lang_a.len() >= lang_b.len(){
        lang_a
    } else {
        lang_b
    }
}

pub fn lifetimes_examples(){
    println!("something about lifetimes");
    println!("When a variable goes out of scope, the value owned by it is dropped");
    println!("Values cannot be dropped if there are still active references to it");
    println!("References to a value cannot outlive the value they refer to");
    let languages = vec![
        "Rust".to_string(),
        "Go".to_string(),
        "TypeScript".to_string(),
    ];
    let result = next_language(&languages, "go");
    println!("{:#?}", result);
    let value = true;
} //-> this is where 'value' goes out of scope.


