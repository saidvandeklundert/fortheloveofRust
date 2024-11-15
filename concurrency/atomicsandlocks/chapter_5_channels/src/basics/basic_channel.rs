/// standard imports
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


/// end
pub fn channel_example(){
    println!("Rust channel example");

    let channel:Channel<String> = Channel::new();

    channel.send("Message 1".to_string());
    let message = channel.receive();
    println!("receive message in main thread: {}", message);


    println!("send messages from the main thread into the channel, use 'stop' or 'q' to quit");
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "q" => break,
            "stop" => break,
            cmd => channel.send(cmd.to_string()),
        }
    }    

}


/// Crude and simple channel to allow multiple threads to exchange messages
/// 
/// Downsides are there is only 1 lock for all channels, the vec can grow endlessly
/// and when it re-sizes, every thread will be waiting.
pub struct Channel<T> {
    queue : Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {

    // Create a new 'channel'
    pub fn new() -> Self{
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready:Condvar::new(),
        }
    }

    // lock the mutex to push a message to the back of the queue and
    // notify one potential receiver ( after unlocking the queue ) by
    // using the Condvar
    pub fn send(&self, message:T){
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    // locks the mutex to pop the next message from the front of the queue,
    // but will use the condition variable to wait if there's no message available yet
    pub fn receive(&self)->T{
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front(){
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}
