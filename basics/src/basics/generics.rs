use num_traits::{ToPrimitive, Float};

#[allow(dead_code)]

pub fn generics_example(){
    println!("some generic examples");
    println!("{}", solve(3.0, 4.0));

    let a: f32 = 3.0;
    let b:f64 = 4.0;
    let a_64 = a.to_f64().unwrap();
    println!("{:#?}", solve(a_64, b));

    let aa: f32 = 3.0;
    let bb: f32 = 4.0;
    println!("{:#?}", solve_generics::<f32>(aa, bb));
    let aaa: f64 = 3.0;
    let bbb: f64 = 4.0;
    println!("{:#?}", solve_generics::<f64>(aaa, bbb));    
    println!("{:#?}", solve_generics(aaa, bbb)); // type annotations from previous line were optional
    println!("{:#?}", solve_generics_version_2(aa, bbb)); 
    println!("{:#?}", solve_generics_version_3(3.4, 1)); 
}

/// this is a regular function written for f64's only
/// if we call this with other types, we always have to take
/// care of the conversion.
fn solve(a:f64, b:f64)->f64{
    (a.powi(2) + b.powi(2)).sqrt()
}

/// Here we have a generic function that will work
/// with any type of Float. You can call this function with
/// f64 or f32, it no longer matters
fn solve_generics<T:Float>(a:T, b:T)->f64{
    let a_64 = a.to_f64().unwrap();
    let b_64 = a.to_f64().unwrap();
    (a_64.powi(2) + b_64.powi(2)).sqrt()
}

/// The above implementation allows for only 1 type to be passed in
/// Either f32 or f64. In order to allow a mix, we have to make
/// the function generic over 2 types.
fn solve_generics_version_2<T:Float, U:Float>(a:T, b:U)->f64{
    let a_64 = a.to_f64().unwrap();
    let b_64 = a.to_f64().unwrap();
    (a_64.powi(2) + b_64.powi(2)).sqrt()
}


/// This one works on all numbers that implement ToPrimitive 
fn solve_generics_version_3<T:ToPrimitive, U:ToPrimitive>(a:T, b:U)->f64{
    let a_64 = a.to_f64().unwrap();
    let b_64 = a.to_f64().unwrap();
    (a_64.powi(2) + b_64.powi(2)).sqrt()
}