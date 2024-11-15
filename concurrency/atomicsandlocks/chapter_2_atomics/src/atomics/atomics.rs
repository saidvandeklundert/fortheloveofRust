use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::time::Duration;
pub fn atomics_example() {
    println!("example working with atomics");
    //stop_flag_example();
    //regular_updates();
}

/// stop flag example

fn stop_flag_example() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // spawn a thread to do some work
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // use this to listen for user input
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {cmd:?}"),
        }
    }
    // inform the background thread it needs to stop
    STOP.store(true, Relaxed);

    // wait until the background thread finishes
    background_thread.join().unwrap();
}

fn some_work() {
    thread::sleep(Duration::from_secs(2));
    println!("background thread doing the things")
}

/// regular updates example
///

fn process_item(item: usize) {
    thread::sleep(Duration::from_millis(220));
    println!("processing item {}", item);
}

fn regular_updates() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // a background thread to process all 10 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
            }
        });
        // the main thread shows status updates every second
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("WORKING ----- ..{n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    })
}

/// lazy initialization
///
/// example where threads use a value that is computed only once.
/// Only the first thread will have to perform the computation. The rest
/// can use the computed value.
///
///

fn get_x() -> u64 {
    // a value of 0 indicates it has never been computed before
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    12313
}
