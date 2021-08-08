# Unite
A small helper macro allowing you to compose existing types into an enum.

## Usage
```toml
[dependencies]
unite = { git = "https://github.com/zerthox/unite", branch = "master" }
```

```rust
use unite::unite;

struct One(bool)
struct Two(i32);
struct Three(f64);

unite! {
	// defines a new enum with a kind for each struct
	pub enum Any {
		One,
		Two,
		Three,
	}
}

fn main() {
	let one = One(true);
	let any = Any::One(one);

	// casts the enum to a specific kind
	if let Some(one) = any.as_one() {
		dbg!(one.0);
	}
}
```
