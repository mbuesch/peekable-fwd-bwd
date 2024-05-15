# Iterator Peekable with multi-forward-peek and multi-backward-peek

[Homepage](https://bues.ch/)

[Git repository](https://bues.ch/cgit/peekable-fwd-bwd.git)

[Github repository](https://github.com/mbuesch/peekable-fwd-bwd)

[crates.io site](https://crates.io/crates/peekable-fwd-bwd)


This is an Iterator Peekable similar to `std::iter::Peekable`, but with additional features:

- Can peek multiple items forwards into the future.
- Can peek multiple items backwards into the past.

This crate is `#![no_std]`, does not heap-allocate and does not contain `unsafe` code.

The wrapped `Iterator::Item` must implement `Clone`.


## Example usage

```rust

use peekable_fwd_bwd::Peekable;
use core::slice::Iter;

let array = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];

const BWD_SIZE: usize = 2; // size of backward peek buffer.
const FWD_SIZE: usize = 8; // size of forward peek buffer.

let mut iter = Peekable::<Iter<i32>, BWD_SIZE, FWD_SIZE>::new(&array);

assert_eq!(iter.next(), Some(&10));
assert_eq!(iter.next(), Some(&11));

// forward peek into the future.
assert_eq!(iter.peek(), Some(&&12));
assert_eq!(iter.peek_nth(0), Some(&&12));
assert_eq!(iter.peek_nth(1), Some(&&13));
assert_eq!(iter.peek_nth(8), None); // FWD_SIZE too small.

assert_eq!(iter.next(), Some(&12));

// backward peek into the past.
assert_eq!(iter.peek_bwd(), Some(&&12));
assert_eq!(iter.peek_bwd_nth(0), Some(&&12));
assert_eq!(iter.peek_bwd_nth(1), Some(&&11));
assert_eq!(iter.peek_bwd_nth(2), None); // BWD_SIZE too small.
```


# Dependencies

- This crate is `#![no_std]`. It does not allocate.
- It only depends on the [arraydeque](https://crates.io/crates/arraydeque) crate without its feature `std`.


# License

Copyright (c) 2024 Michael Büsch <m@bues.ch>

Licensed under the Apache License version 2.0 or the MIT license, at your option.
