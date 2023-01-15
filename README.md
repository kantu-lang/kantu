# Kantu

Kantu (rhymes with "onto") is a programming language for writing highly reliable programs.

- Secure by default--no file, network, or environment access, unless explicitly enabled
- Memory safe
- No runtime exceptions
- Guaranteed termination (no infinite loops/recursion)
- Arbitrary preconditions and postconditions can be checked at compile-time

Kantu is [pure](https://en.wikipedia.org/wiki/Purely_functional_programming) and [total](https://en.wikipedia.org/wiki/Total_functional_programming), and supports [dependent types](https://en.wikipedia.org/wiki/Dependent_type).

## Why Kantu?

With Kantu, you can specify extremely precise properties
about the behavior of your code, and the compiler will
check those properties at compile-time.
Think "type system on steroids."

We are all familiar with compile-time checks in
some form or another.
Among the most famous is type checking.
Consider the following Rust code

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
Thus, we can say that the compiler _checks type-correctness_.

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

How? Through type checking!

"Wait, type checking? Isn't that the same as, say, Rust?"

The key difference is that Kantu supports [dependent types](https://en.wikipedia.org/wiki/Dependent_type), which allow types to depend on
_terms_.
This lets us get much more precise with our types.

For example, consider the Java generic type `ArrayList<T>`.
What if we wanted to extend the type to include the
length of the list?
Something like `ArrayList<T, n>`, where `T` is a type
and `n` is a number.

Well, it turns out this is unfortunately impossible.
Java generic types can only depend on
_types_ (like `T`), not _terms_ (like `n`).
However, this is possible with Kantu's dependent types!

Returning to the binary search example,
we can define a (dependent) type that represents
what it means for the binary search to be "correct":

```kantu
// Assume that `binary_search` has already
// been defined above.

use std.list.*;
use std.eq.*;

let BinarySearchOutputCorrect = fun _(
    list: List(Nat),
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

let BinarySearchCorrect = forall(
    list: List(Nat),
    ascending: Ascending(list),
    target: Nat,
) {
    BinarySearchOutputCorrect(list, target)
};
```

Then, we can prove that this property holds by
writing a term of type `BinarySearchCorrect`:

```kantu
use std.ascribe;

let binary_search_correct = ascribe(
    BinarySearchCorrect,
    fun _(
        list: List(Nat),
        ascending: Ascending(list),
        target: Nat,
    ): BinarySearchOutputCorrect(list, target) {
        // <Write implementation here>
    }
);
```

> If you don't understand the above code, no worries!
> The [Language Overview](./docs/overview.md) should make
> everything make sense.

This might seem more verbose than the original Rust example
(i.e., the one with no runtime checks), and it is.
However, this has the benefit of being _verifiably correct_!

Across all programming languages, programs usually to obey this law:

![Code safety is inversely proportional to brevity](./docs/assets/code_safety_vs_brevity.jpg "Code safety vs. brevity")

Kantu's selling point is that it empowers _you_, the programmer,
to choose where on the curve you want to be.
When you don't care much about correctness and just want to
write concise, readable code, you can!
When you desperately want to ensure at all costs that
a given piece of code is correct, you can!

### Why not Coq/Lean/Agda/Idris?

Actually, all those are perfectly fine, too.
Maybe even _better_, if you're working on a
"highly mathy" project.
For example, the Four Color Theorem has been
proven in [Coq](https://github.com/coq-community/fourcolor),
but writing the same proof in Kantu would be tedious,
if not impossible.

However, all those languages are, well, ...big.
They contain a lot of features--more than you will
probably ever need for general-purpose programming.

Kantu deliberately aims to be as simple as possible,
which not only makes it easier to learn, but also
easier to create tooling for.

## Guides

- [Language Overview](./docs/overview.md)

## License

Kantu is distributed under both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.

```

```
