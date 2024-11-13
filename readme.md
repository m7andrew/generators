
# Generators

A Rust library to make writing generators straightforward. As with [Python generators](https://www.geeksforgeeks.org/generators-in-python/), the `generator` macro provides a nice way to write an [iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html) as a single function.

```rust
#[generator]
fn numbers() -> i32 {
  yield 1;
  yield 2;
  yield 3;
}
```

This library supports: **try expressions** `?`, **early returns**, **recursion**, and the `yield_from` macro.

## Installation

This library currently requires [nightly Rust](https://rust-lang.github.io/rustup/concepts/channels.html) as it uses coroutine features not yet surfaced to stable Rust. To use in your project, add `generators` as a dependency to your `Cargo.toml` file:

```toml
[dependencies]
generators = { git = "https://github.com/m7andrew/generators" }
```

You will then need to add these feature flags to the top of your `main.rs` or `lib.rs` file:

```rust
#![feature(coroutines, coroutine_trait, try_trait_v2)]
```

## Usage

**Generators** are created with the `#[generator]` macro. This takes a function and transforms it into a function that returns an [iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html). Because generators create iterators, your generator functions must have explicit return types.

**Recursive generators** are possible via the `boxed` attribute: `#[generator(boxed)]`. This extra attribute is needed because in order for coroutines (like generators) to be recursive in Rust, they must be [boxed](https://doc.rust-lang.org/std/boxed/index.html) and thus heap allocated.

**Returns** within a generator act as a final yield. A `return` will first yield its value and then end the generator on the next iteration (returning `None`). This is useful for early returns. An empty `return` will simply end the generator without first yielding something.

**To yield from** another generator or iterator, use the `yield_from!` macro. This is similar to Python's `yield from`.

For examples, please take a look at the [tests](https://github.com/m7andrew/generators/blob/master/tests/tests.rs).

## Future Rust

While Rust generators have been in the works for years, they are far from completion. In the 2024 edition of Rust, [gen blocks](https://github.com/rust-lang/rust/issues/117078) are partially implemented and may be usable... someday. Currently, try expressions [don't work](https://github.com/rust-lang/rust/issues/117486) as laid out in the [RFC](https://github.com/rust-lang/rfcs/blob/master/text/3513-gen-blocks.md) and other design decisions have yet to be hammered out. So until then, this library aims to provide a nice way to use some of Rust's cutting-edge features.

## Limitations

1) As a macro, error messages will not be as nice as those coming from native language features. Sorry.

2) [Implicit returns](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html#functions-with-return-values), or final expression returns will throw a compile error. Besides the semantic ambiguity, there is no way (that I know of) for macros to detect implicit returns. So stick with `yield` and `return` when writing your generators.

3) Async generators are currently not implemented due to the complexity it would add.
