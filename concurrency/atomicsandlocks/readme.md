## Notes from Atomics and locks


Threads may run untill the end of the program. Therefore, the `thread::spawn` function has a `static` lifetime bound on its argument type. That means it only accepts functions that may be kept around forever. 

When sharing data between two threads where neither thread is guaranteed to outlive the other, neither of them can be the owner of that data. Any data shared between them will need to live as long as the
longest living thread.

There are several ways to create something that is not owned by a single thread:
- `static` values: they are owned by the entire program. A `static` has a constant initializer, is never dropped and even exists before `main` starts. Every thread can borrow from it since it is guaranteed to always exist.
- leaking using something like `let x :&'static [i32;3]=Box::leak(Box::new([1,2,3]))`. The `'static` lifetime does not mean since the start of the program, rather, it means it lives till the end of the program. This value is never dropped and thus, if we do it too oftentimes, we slowly run out of memory.
- reference counting


Atomics in computer science is used to describe a process which is indivisible. Atomic operations are the main building block for anything involving multiple threads. All the other concurrency primitives, such as mutexes and condition variables, are implemented using atomic operations.

### Closures

Closures are important to understand in Rust as they are used or expected to be used in many places. Apart from their heavy use in iterators, they are also commonly used in the context of threads.

Closures are functions that can capture the enclosing environment. The following are a few examples to illustrate closures and their syntax. In the examples, `x` is the outer variable that is enclosed. In regular functions, this variable would not be accessible:

```rust
let x = 1;
// single line closure:
|| 1 +x ;
// multine closure, enclosed in '()':
(||{
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
let z_annotated = |input_value:i32| ->i32  {x * input_value};
let result = z_annotated(12);
println!("result from annotated closure: {:?}", result);
```


