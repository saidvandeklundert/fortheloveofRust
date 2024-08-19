## Modules


Modules group together related code.

### Summary

```rust
mod content; // pull in module

use content::Catalog;  // shorten access to using the Catalog struct
```

There are three different ways to create modules.

### 1: Create a mod in an existing file

Most appropriate when you have a really large file with a lot of things going on.

```rust
mod content {
    pub enum Media { /* fields */ }

    pub stuct Catalog { /* fields */ }
}

fn main() {
    let catalog = content::Catalog::new();
}
```
Here, the `mod` keyword is used to create the module inside the file.

### 2: Create a module in a new single file in the same folder:

Most appropriate when you want a separate module to organize code, but it doesn't need to span several files.

`src/content.rs`:
```rust
pub enum Media { /* fields */ }

pub stuct Catalog { /* fields */ }
```

`src/main.rs`:
```rust
mod content;

fn main() {
    let catalog = content::Catalog::new();
}
```

Here, the `mod` keyword is used to pull in the module. 

### 3: Spread code out among several separate files in a new folder

Most appropriate when you have a large module and most used in real projects.

Has a few confusing parts:

- 1: every _file_ and _folder_ makes its own separate module

The following files and folders will give us these modules:
`content/media.rs`:     -> media module
`content/catalog.rs`:   -> catalog module
`content/mod.rs`:       -> content module
`main.rs`:              -> root module

- 2: you cannot directly import nested modules

From the root module, `main.rs`, you cannot import the `media` module directly. Say there is something in `media.rs` that you want to import in `main.rs`, you would need to do the following:
`content/media.rs`:     -> `enum media`
`content/mod.rs`:       -> `pub mod media` -> imports and exports everything from the media module
`main.rs`:              -> `mod content` -> imports everything from the content module

- 3: using something from the same module in another file might require you to use `super`

In the case with `catalog` and `media`, the importing and exporting is happening inside `mod.rs`. This is the parent module to both `catalog` as well as `media`. If, in `catalog`, you want to use a struct that is defined in `media`, you can use the following:
```rust
use super::media::Media;
```

`src/content/media.rs`:
```rust
pub enum Media { /* fields */ }
```

`src/content/catalog.rs`:
```rust
pub stuct Catalog { /* fields */ }
```

`src/content/mod.rs`:
```rust
pub mod media;
pub mod catalog;
```

`src/main.rs`:
```rust
mod content

fn main() {
    let catalog = content::catalog::Catalog::new();
}
```