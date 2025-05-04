# Kantu

> ## 🚧 The Kantu project has been discontinued. 🚧
>
> Both the code and the documentation are incomplete.
> Proceed at your own risk.

## Introduction

Kantu (rhymes with "onto") is a programming language for writing highly reliable programs.

- Secure by default--no file, network, or environment access, unless explicitly enabled
- No runtime exceptions
- Guaranteed termination (no infinite loops/recursion)
- Arbitrary preconditions and postconditions can be checked at compile-time

Kantu is [pure](https://en.wikipedia.org/wiki/Purely_functional_programming) and [total](https://en.wikipedia.org/wiki/Total_functional_programming), and supports [dependent types](https://en.wikipedia.org/wiki/Dependent_type).

## Why Kantu?

In short, you should use Kantu if you want a language that...

- ...lets you specify precise behavioral guarantees
  - Kantu's dependent types give you far more precision than types in Rust, Go, Swift, C++, TypeScript, Kotlin, Python, Ruby, Perl, etc.
- ...is relatively simple
  - Kantu is far simpler than Coq, Lean, Agda, and Idris.

For more details, please check out the [Why Kantu?](./docs/why_kantu.md) page.

## Guides

- [Language Overview](./docs/overview.md)

## License

Kantu is distributed under both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
