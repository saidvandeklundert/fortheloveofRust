use std::time::Duration;
#[allow(unused)]
use std::{
    cell::{Cell, RefCell, UnsafeCell},
    collections::VecDeque,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::NonNull,
    rc::Rc,
    sync::{
        atomic::{Ordering::*, *},
        *,
    },
    thread::{self, Thread},
};

use std::sync::Mutex;

/// To ensure a locked mutex can only be unlocked by the thread that locked it, it does
/// not have an unlock() method. Instead, its lock() method returns a special type
/// called a MutexGuard. 
/// The guard represents the guarantee we have locked the mutex. 
/// Unlocking the mutex is done by dropping the guard.
/// 
/// 
/// In the function below, we have a mutex protecting an integer.
/// 10 threads are spawned and increment the integer 100 times.
/// Every thread locks the mutex and mutates the integer through the guard.
/// Af the end of each thread, the guard goes out of scope and it is dropped.
/// 
/// Effectively, through the mutex, incrementing the integer 100 times becomes
/// an atomic operation.
pub fn mutex_example() {
    let n = Mutex::new(0);
    println!("{:#?}", n);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    println!("{:#?}", n);
    assert_eq!(n.into_inner().unwrap(), 1000);
}

#[derive(Debug)]
struct GiantProgram{
    module_a: Box<String>,
    module_b: Box<String>,
    module_c: Box<String>,
    module_d: Box<String>,
}

impl GiantProgram{
    pub fn new()->Self{
        
        GiantProgram {
            module_a: Box::new("".to_string()), 
            module_b: Box::new("".to_string()),
            module_c: Box::new("".to_string()),
            module_d: Box::new("".to_string()),
        }
    }
}

/// Four threads work together and write a program
pub fn another_mutex_example(){
    let mut program = Mutex::new(GiantProgram::new());
    println!("{:#?}", program);
    thread::scope(|s| {
        for i in 0..4 {
            s.spawn(|| {
                let module_result = build_module();
                let mut guard = program.lock().unwrap();
                //*guard.module_a.push_str("asa");
                println!("{:#?}", guard);
                guard.module_a.push_str(&module_result);
                
            });
            
        }
    });   
    println!("{:#?}", program); 
    

}

fn build_module()->String{
    let id = thread::current().id();
    println!("thread {:?} putting in the work", id);
    thread::sleep(Duration::from_millis(5));
    
    String::from(format!("{:?} wrote something", id))
}


fn build_counter() {
    let id = thread::current().id();
    let mut counter = 0;
    
    loop {
        println!("thread id {:?} counter {}", id, counter);
        counter +=1;
        thread::sleep(Duration::from_millis(150));
    }    
}

fn build_interface_state() {
    let id = thread::current().id();
    let mut counter = 0;
    
    loop {
        println!("thread id {:?} interface state {}", id, counter);
        counter +=1;
        thread::sleep(Duration::from_millis(250));
    }    
}

#[derive(Debug)]
pub struct CliOutput{
    counter_a:i32,
    counter_b:i32,
}

impl CliOutput{

    pub fn new()->Self{
        CliOutput{counter_a:0,counter_b:0}
    }
}

fn displayer(){
    let id = thread::current().id();
    loop {
        println!("thread id {:?} displaying CLI output", id);
        thread::sleep(Duration::from_millis(500));
        
    }       

}

pub fn build_cli_output(){
    let mut cli_output = Mutex::new(CliOutput::new());
    let thread_1 = thread::Builder::new().name("thread1".to_string()).spawn(build_counter);
    let thread_2 = thread::Builder::new().name("thread2".to_string()).spawn(build_interface_state);    
    let thread_3 = thread::Builder::new().name("thread3".to_string()).spawn(displayer); 
    println!("{:?}",cli_output);
    thread_1.expect("REASON").join().unwrap();
    thread_2.expect("REASON").join().unwrap();
    thread_3.expect("REASON").join().unwrap();
}