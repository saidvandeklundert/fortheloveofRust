There are 3 ways of dealing with the Result enum:
- try operator (`?`):
- match arms
- `expect` or `unwrap`

Some guidelines on when to use one over the other:
1. Use a `match` or `if let` statement when you can meaningfully deal with an error.
2. call `unwrap()` or `expect()` on the Result when you are debugging OR you want to crash on an error
3. Use the try operator (`?`) to unwrap or propagate the Result for when you do not have any way to handle the error in the current function


The third options is the most difficult to understand. Look at the following:
```rust
fn read_config_file() -> Result<String, Error> {
    fs::read_to_string("config.json")
}

fn get_config() -> Result<String, Error> {
    let config = read_config_file()?;

    Ok(config)
}

fn main() -> Result<(), Error> {
    let config = get_config()?;

    println!("Got a config: {}", config);

    Ok(())
}
```

Applying the try operator on the get_config function will let the error 'bubble up' inside the calling function.

Say this main function would be wrapped in a match, this match would take into consideration the try resulting in an error in this case.