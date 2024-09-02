/// Box Smart Pointer
/// 
/// Rust by default stores everything on the stack.
/// Using the Box smart pointer, you can put something on the heap.
/// 
/// Smart pointer:
/// - 'special' capabilities
/// - not just a simple reference
/// 
/// Simple pointer:
/// - stores memory address
/// - indicated by &
/// - no special abilities

#[allow(dead_code)]

pub fn smart_pointers_example(){
    println!("smart pointers");
    let x :i32 = 5; // stack allocated value
    let y = Box::new(x); // putting the value on the heap and taking a smart pointer to it
    let z = &x; // pointer to something on the stack
    example_recursive_type();
}


/// Stack allocated items have to be known at compile time
/// The following enum is a recursive type and presents a
/// prolem since the size is not known in advance. There can be n-number 
/// of List values contained inside list:


///The following give this error: 'error[E0072]: recursive type `List` has infinite size'
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nill,
}
fn example_recursive_type(){
    let example = Box::new(List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3,Box::new(List::Nill)))))));

}


// Example with huge data as well as multiple types that sit behind a smart pointer
struct HugeData;
struct SmallData;
trait Storage{}

impl Storage for HugeData{}
impl Storage for SmallData{}

fn example_huge_data(){
    let data_1=HugeData;
    let data_2=Box::new(HugeData);
    let data_3 = data_1; // during the move, the entire data is copied on the stack
    let data_4 = data_2; // during the move,  the box pointer is copied
    // Example mixing types on a Vector:
    let data_5 = Box::new(SmallData);
    
    // following failes due to the mixing of types (HugeData and SmallData)
    //let data_fail = vec![Box::new(data_3), data_4, data_5];
    
    // following passes since the Vector now accepts any type that implements
    // the storage trait:
    let data: Vec<Box<dyn Storage>> = vec![Box::new(data_3), data_4, data_5];

}