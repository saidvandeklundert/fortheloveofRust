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

fn f() {
    println!("Hello from f!");
    let id = thread::current().id();

    println!("Currently using thread id {:?}", id);
}

pub fn threads_examples() {
    println!("Examples on using threads!");
    let thread_1 = thread::spawn(f);
    let thread_2 = thread::spawn(f);
    println!("Hello from Threads examples, currently executing under main!");

    thread_1.join().unwrap();
    thread_2.join().unwrap();

    // instead of passing a name into the thread,
    // more often a closure is passed in:
    let numbers = vec![1, 2, 3, 4];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
    // the above 'move' will move all the values into the closure
    // had we not used a move, we would have passed a reference
    // Because of this move, we can no longer refer to the 'numbers' binding.

    // We can create a new binding with the same name though.
    // getting a value back from a thread is done by returning it from the closure:
    let numbers = Vec::from_iter(00..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("Average: {average}");

    // The Rust standard library provides the std::thread::scope function to spawn such
    // scoped threads. It allows us to spawn threads that cannot outlive the scope of the
    // closure we pass to that function, making it possible to safely borrow local variables.
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        // the s represents the scope
        println!("{:#?}", s);
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
    /// in the above we call std::thread::scope with a closure that is directly
    /// executed. We use s to spawn threads. The closure in this case can borrow
    /// local variables.
    /// When the scope ends, all threads that have not been joined yet are
    /// automatically joined.
    ///
    /// ARC stands for atomically reference counted
    // First we put an array into a new allocation together with a reference counter
    // that starts at 1:
    let a = Arc::new([1, 2, 3]);
    // when we clone the arc, the ref count is upped by 1:
    let b = a.clone();
    // both threads can now get their own arc and share access to the array
    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
    // the threads drop the arc and decrement the ref count when they finish.
    // the arc drop that sets the ref count at 0 will drop and deallocate the array.
    println!("");
}
