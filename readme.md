

# `simple_predicates` -- a library for modelling simple boolean predicates

Basic usage is to implement `Eval` for your boolean literal type, then construct `Expr`s using the boolean literals. To evaluate the truth value of the expression, call the `eval` method, passing in any contextual data required to resolve the predicate.



# Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
simple_predicates = "0.1"
```

# License

simple_predicates is licenced with the [MIT license](/license-mit.md) or the [Apache version 2.0 license](/license-apache.md), at your option.

