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

mod basics;
use basics::basic_threads::threads_examples;
use basics::cond_var::cond_var_example;
use basics::mutex::{mutex_example, another_mutex_example, build_cli_output};
fn main() {
    let x = 1;
    // single line closure:
    || 1 + x;
    // multine closure, enclosed in '()':
    (|| {
        println!("we are in a closure now");
        println!("we have access to enclosing variables, like x: {}", x);
    })();
    // we can assign a closure to a variable:
    let y = || x * 2;
    println!("y is {:?}", y());

    // closures can take input:
    let z = |input_value| x * input_value;
    let result = z(12);
    println!("result: {:?}", result);

    // closures can be annotated:
    let z_annotated = |input_value: i32| -> i32 { x * input_value };
    let result = z_annotated(12);
    println!("result from annotated closure: {:?}", result);
    threads_examples();
    cond_var_example();
    mutex_example();
    another_mutex_example();
    build_cli_output();
}
