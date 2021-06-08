

# `simple_predicates` -- a library for modelling simple boolean predicates


A predicate is a formula that can be evaluated to true or false as a function of the values of the variables that occur in it. In `simple_predicates`, the variables denoted by a user-chosen type satisfying the `Eval` trait (which requires `Clone` and `PartialEq`.) The `Eval` trait also provides an associated `Context` type which can be used to provide contextual data needed to resolve the variables. The `Expr`, `Cnf`, and `Dnf` types can be used to construct evaluable expressions.


# Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
simple_predicates = "0.2"
```

# Example Usage

Lets say we want to write boolean expressions denoting the possibility of a `Vec` relating to some `u32` values according to the `Vec::contains` method. We start by implementing `Eval`:

```rust
use simple_predicates::Eval;

// Wrapper struct needed because `u32` & `Eval` are both foreign.
#[derive(Clone, PartialEq)]
struct Contains(pub u32);


impl Eval for Contains {
    type Context = Vec<u32>;

    fn eval(&self, data: &Self::Context) -> bool {
        // Evaluate the u32 by checking if it is in the `Vec`.
        data.contains(&self.0)
    }
}

```

Now we can check arbitrary containment conditions on a given `Vec` like so:

```rust
use simple_predicates::Eval;
use simple_predicates::Expr::*;

let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];

// `eval` is `true` if input contains 4 but not 5.
let expr = And(
    Box::new(Var(Contains(4))),
    Box::new(Not(Box::new(Var(Contains(5))))));

assert!(expr.eval(&items));

```

# Cnf / Dnf

For more complex expressions, the nesting of `And` and `Or` expressions can ge very tedious, so the `Cnf` and `Dnf` types are provided to simplify their handling.

The `Cnf` type represents the [Conjunctive Normal Form](https://en.wikipedia.org/wiki/Conjunctive_normal_form) of a boolean expression; a set of expressions which are `And`ed together. The `Dnf` type represents the [Disjunctive Normal Form](https://en.wikipedia.org/wiki/Disjunctive_normal_form) of a boolean expression; a set of expressions which are `Or`ed together.

The `Cnf` and `Dnf` types can only be used if the variable type implements `Eq` and `Hash`. They have identical APIs, so the examples below are representative of either.

# Examples

A `Cnf` can be constructed from an Expr, using the `From` trait:

```rust
use simple_predicates::Eval;
use simple_predicates::Expr::*;

let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];

let cnf = Cnf::from(
    And(
        Box::new(Var(Contains(4))),
        Box::new(Not(Box::new(Var(Contains(5)))))));

assert!(cnf.eval(&items));

```


A `Cnf` can also be constructed from anything that emits Exprs with the `IntoIterator` trait:

```rust
use simple_predicates::Eval;
use simple_predicates::Expr::*;

let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];

let cnf = Cnf::from(vec![
    Var(Contains(4)),
    Not(Box::new(Var(Contains(5)))),
]);

assert!(cnf.eval(&items));

```


# License

simple_predicates is licenced with the [MIT license](/license-mit.md) or the [Apache version 2.0 license](/license-apache.md), at your option.


