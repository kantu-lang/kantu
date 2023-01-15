# Kantu

Kantu (rhymes with "onto") is a programming language for writing highly reliable programs.

- Secure by default--no file, network, or environment access, unless explicitly enabled
- Memory safe
- No runtime exceptions
- Guaranteed termination (no infinite loops/recursion)
- Arbitrary preconditions and postconditions can be checked at compile-time

Kantu is [pure](https://en.wikipedia.org/wiki/Purely_functional_programming) and [total](https://en.wikipedia.org/wiki/Total_functional_programming), and supports [dependent types](https://en.wikipedia.org/wiki/Dependent_type).

## Why Kantu?

Kantu allows you to enforce extremely precise statically checked
conditions on your code.

We are all familiar with static (i.e., compile-time) checks in
some form or another.
For example, consider the following Rust code

```rust
/// Returns the index of `target` in `list`,
/// where `list` is a strictly ascending list of integers.
/// Returns `None` if `target` is not in `List`.
pub fn binary_search(list: &[u32], target: u32) -> Option<usize> {
    // <business logic goes here>
}
```

In this example, the compiler will check that

- Any invocation of
  `binary_search` has 2 arguments, of type `&[u32]` and `u32`, respectively.
- The implementation of `binary_search` returns a value of type `Option<usize>`.

If either of these conditions are not satisfied, the compiler will emit
a type error.

However, apart from type-correctness, the compiler doesn't check
much else.
For example, this obviously erroneous implementation
compiles perfectly fine:

```rust
/// Returns the index of `target` in `list`,
/// where `list` is a strictly ascending list of integers.
/// Returns `None` if `target` is not in `List`.
pub fn binary_search(list: &[u32], target: u32) -> Option<usize> {
    // WRONG
    return None;
}
```

Furthermore, even if the implementation was correct, the compiler
would not check that every invocation passed a sorted list.
For example, this obviously erroneous invocation
compiles perfectly fine:

```rust
fn main() {
    let out_of_order = vec![3, 4, 7, 1, 3];
    let index = binary_search(&out_of_order, 1);
    println!("{:?}", index);
}
```

The problem is that the _preconditions_ and _postconditions_ checked
by the compiler are too weak.

- The compiler checks the precondition that the the first argument
  is of type `&[u32]`, but it does NOT check that it is strictly
  ascending.
- The compiler checks the postcondition that the output is of the type
  `Option<usize>`, but it does NOT check that it is the
  (possibly non-existent) index of `target`.

Of course, we could check these conditions at runtime, but that
would incur overhead:

```rust
/// Returns the index of `target` in `list`,
/// where `list` is a strictly ascending list of integers.
/// Returns `None` if `target` is not in `List`.
pub fn binary_search(list: &[u32], target: u32) -> Option<usize> {
    // START Precondition check
    for i in 0..list.len() {
        let j = i + 1;
        if j >= list.len() {
            break;
        }
        assert!(list[i] < list[j]);
    }
    // Precondition END check

    // <Insert business logic here>

    // START Postcondition check
    match out {
        Some(i) => assert_eq!(target, list[i]),
        None => assert!(!list.contains(&target)),
    }
    // END Postcondition check

    return out;
}
```

As shown above, adding runtime checks increases `binary_search`'s runtime
to O(n), completely defeating the purpose of binary search
(At this point, a linear search would probably be faster)!

**With Kantu, you can perform these checks at _compile time_!**

```kantu
let binary_search = fun _(
    list: List(Nat),
    target: Nat,
): Option(Nat) {
    // <Write business logic here>
};

use std.list.*;
use std.eq.*;

// Correctness definition
let binary_search_correctness = fun _(
    list: List(Nat),
    ascending: Ascending(list),
    target: Nat,
): Type {
    match binary_search(list, target) {
        none => Not(In(Nat, target, list)),
        some(i) => Eq(
            Option(Nat),
            try_get(Nat, list, i),
            some(target),
        ),
    }
};

// Correctness proof
let binary_search_correct = fun _(
    list: List(Nat),
    ascending: Ascending(list),
    target: Nat,
): binary_search_correctness(list, ascending, target) {
    // <Write proof here>
};
```

## Guides

- [Language Overview](./docs/overview.md)

## License

Kantu is distributed under both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
