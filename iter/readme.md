

Iterators are a separate structure from the thing we are iterating over.

The iterator itself is a struct and has several methods. The most well known method is the `next()` method. The next method returns an `Option` enum:
```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Iterators in Rust are lazy.

There are 3 main iterator methods in Rust:
- `iter()`: gives you a read-only reference
- `iter_mut()`: gives you a mutable reference
- `into_iter()`: gives you ownership of each element *unless* called on a mutable ref to a vector

`into` somewhere in a function generally implies ownership is acquired.

Instead of instantiatin an iterator and calling `next` yourself, usually people turn to these options:
1. Using a for loop:
```rust
fn print_elements(elements:&Vec<String>){
    for element in elements{
        println!("{}", element);
    }
}
```
This `for loop` will repeatedly call `next` for you and automatically unwrap the Option. When a `None` is encountered, the `for loop` will break automatically.

2. Using iterator adaptors and consumers like `for_each`, `collect`, `map`, etc.

```rust
fn iterator_adapter_and_consumers(elements: &Vec<String>) {
    // iterator consumer 'for_each' calls the next untill depleted:
    elements.iter().for_each(|element| println!("{}", element));
    // iterator adapters add an additional processing step:
    elements
        .iter()
        .map(|element| format!("{} {}", element, element))
        .for_each(|element| println!("{}", element));
}
```
The `for_each` takes a closure, which is specified using `| |`. In the closure, the first entry is the function argument, the second is the function body.