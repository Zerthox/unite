# Unite
A small helper macro allowing you to compose existing types into an enum.

## Usage
```toml
[dependencies]
unite = { git = "https://github.com/zerthox/unite", branch = "master" }
```

```rust
use unite::unite;

pub struct One(bool);
pub struct Two(i32);
pub struct Three(f64);

unite! {
    // defines a new enum with a variant for each struct
    pub enum Any { One, Two, Three }
}
```

### Renaming
By default the enum variants use the same name as the type, but renaming is possible.

```rust
unite! {
    enum Foo {
        SameName,
        Renamed = i32,
    }
}
```

### Helper functions
The generated enums come with helper functions to access their variants with ease.
Variant names are automatically converted into `snake_case` for the function names.

```rust
fn foo(any: Any) {
    // checks whether the enum is a specific variant
    let is_one: bool = any.is_one();

    // attempts to cast the enum to a specific variant
    let as_two: Option<&Two> = any.as_two();
    let as_three_mut: Option<&mut Three> = any.as_three_mut();
}
```
