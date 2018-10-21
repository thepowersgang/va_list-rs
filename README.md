Rust implementation of C's `va_list` type

# Overview
This crate provides a rust `VaList`type, which is binary-compatible for the C `va_list` type. It is intended to allow rust code to provide the complex logic of variable argument functions.

# Example
```rust
extern crate va_list;
use va_list::VaList;

extern "C" print_ints_va(count: u32, mut args: VaList)
{
	for i in (0 .. count) {
		println!("{}: {}", i, args.get::<i32>());
	}
}
```

# Status
- x86-64 linux/ELF ABI (aka System-V) : Tested in the wild, works relatively well
- x86 linux/ELF ABI (sys-v) : Unit tested only


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
