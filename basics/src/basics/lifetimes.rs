///
/// Lifetimes:
/// - When a variable goes out of scope, the value owned by it is _dropped_
/// - Values cannot be dropped if there are still active references to it
/// - References to a value cannot outlive the value they refer to
/// 
/// 
pub fn lifetimes_examples(){
    println!("something about lifetimes");
    println!("When a variable goes out of scope, the value owned by it is dropped");
    println!("Values cannot be dropped if there are still active references to it");
    println!("References to a value cannot outlive the value they refer to");

    let value = true;
} //-> this is where 'value' goes out of scope.