# `periodic-array`

`periodic-array` is a Rust library providing a fixed-size array structure that allows fast access to its elements under the assumption of periodicity. Accessing an index beyond the array's bounds will wrap around to the beginning, effectively treating the array as cyclic or infinite.

## Features

- **Performance:** Utilizes unsafe operations (`get_unchecked` and `get_unchecked_mut`) for fast access without bounds checking. The modulo operation ensures there is never an out-of-bounds access.
- **Macro Support:** Includes the `p_arr!` macro for easy and readable array creation.
- **Conditional Copy Derivation:** Optional `Copy` trait derivation hidden behind feature flag to ensure arrays are not accidentally copied.

## Usage

```rust
use periodic_array::p_arr;

let pa = p_arr![1, 2, 3];
assert_eq!(pa[1], 2);
assert_eq!(pa[4], 2); // Access beyond the length wraps around
