fn basic_iterator(colors: &Vec<String>) {
    let mut colors_iter = colors.iter();

    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
    println!("{:#?}", colors_iter.next());
}

fn print_elements(elements: &Vec<String>) {
    for element in elements {
        println!("{}", element);
    }
}

fn iterator_adapter_and_consumers(elements: &Vec<String>) {
    // iterator consumer 'for_each' calls the next untill depleted:
    elements.iter().for_each(|element| println!("{}", element));
    // iterator adapters add an additional processing step:
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}

fn shorten_strings(colors: &mut Vec<String>) {
    colors.iter_mut().for_each(|element| element.truncate(1));
}

fn shorten_strings_vector_slice(colors: &mut [String]) {
    colors.iter_mut().for_each(|element| element.truncate(1));
}

// have the compuler figure out the return type
fn to_uppercase(collors: &[String]) -> Vec<String> {
    collors
        .iter()
        .map(|element| element.to_uppercase())
        .collect()
}

/// collect() can take anything iterable, and turn it into a relevant collection.
/// two more ways to specify the return type
/// 1: annotate the result binding
/// 2: annotate the collect call
fn to_uppercase_alt(collors: &[String]) -> Vec<String> {
    let result: Vec<String> = collors
        .iter()
        .map(|element| element.to_uppercase())
        .collect::<Vec<String>>(); // turbofish
    result
}
/// Why use a vector-slice of String over '&Vec<String>'?
///
/// This is because  when the type is a vector-slice, you can pass
/// in the entire Vector or a portion of the vector
///
fn iterator_adapter_and_consumers_vec_slice(elements: &[String]) {
    // iterator consumer 'for_each' calls the next untill depleted:
    elements.iter().for_each(|element| println!("{}", element));
    // iterator adapters add an additional processing step:
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}

/// Calling into_iter gives you ownership. This allows you to move values
/// from one vector to another vector
fn move_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el))
}

fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}


// map_or will return the first value specified if the Option is None
// the value otherwise 
fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}


fn use_filter(elements: Vec<String>, filter_for: &str) ->Vec<String>{
    let new_elements = elements
        .into_iter()
        .filter(|el| el.contains(filter_for))
        .collect::<Vec<_>>();
    return new_elements;
    
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    basic_iterator(&colors);
    print_elements(&colors);
    iterator_adapter_and_consumers(&colors);
    iterator_adapter_and_consumers_vec_slice(&colors[1..3]); // pass in a Vector slice
    shorten_strings(&mut colors);
    shorten_strings_vector_slice(&mut colors[1..3]);
    print_elements(&colors);
    println!("{:#?}", to_uppercase(&colors));
    println!(
        "{:#?}",
        explode(&vec!["something".to_string(), "else".to_string()])
    );
    println!(
        "{:#?}",
        find_color_or(
            &vec!["red".to_string(), "blue".to_string(), "green".to_string()],
            "purple",
            "orange"
        )
    );
    println!(
        "{:#?}",
        use_filter(vec!["z".to_string(), "a".to_string()], "z")
    );    
}
