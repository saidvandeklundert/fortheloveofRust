fn main() {
    println!("Hello, world!");
    /// Functions and Methods:
    /// 
    enum Shape {
        Rectangle{ width:f64, heigth: f64 },
        Circle{ radius:f64 },
    }

    impl Shape{
        pub fn area(&self)->f64{
            match self {
                Shape::Rectangle {width, heigth}=>width *heigth,
                Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            }
        }
    }

    /// Function pointers:
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    // Explicit coercion to `fn` type is required...
    let op: fn(i32, i32) -> i32 = sum;
    let result = op(3,3);
    println!("{}", result);

    /// Closures
    let amount_to_add = 3;
    let add_n = |y| {        
        amount_to_add + y
        
    };
    
    println!("{} {} {} {}", add_n(3),add_n(3),add_n(3),add_n(3))   ; 
    // Closure that continuously changes the enclosed value:
    let  mut amount_to_add = 3;
    let mut add_n = |y| {
        // a closure capturing `amount_to_add`
        amount_to_add += y;
        amount_to_add
    };
    
    println!("{} {} {} {}", add_n(3),add_n(3),add_n(3),add_n(3))
   
}
