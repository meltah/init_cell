[![Crates.io](https://img.shields.io/crates/v/init_cell.svg)](https://crates.io/crates/init_cell)

# init_cell

A cell which can be unsafely initialized or interiorly mutated, but safely accessed.

This is mostly intended for use in statics. The cell is safe to access, but must be initialized before any access. There is no synchronization to ensure initialization is observed, so you should initialize at the beginning of the main function or using something like the `ctor` crate.

## Example

```rust
use init_cell::InitCell;

// SAFETY: We will initialize the cell before using it.
pub static MY_VAL: InitCell<Vec<i32>> = unsafe { InitCell::new() };

fn main() {
	// SAFETY: Nothing is accessing the cell.
	unsafe {
		InitCell::init(&MY_VAL, vec![1, 2, 3]);
	}
	assert_eq!(MY_VAL.iter().sum::<i32>(), 6);

	// The cell can be mutated, too, which drops the previous value.
	unsafe {
		InitCell::set(&MY_VAL, vec![4, 5, 6]);
	}
	assert_eq!(MY_VAL.iter().sum::<i32>(), 15);
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
