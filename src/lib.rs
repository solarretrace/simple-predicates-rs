////////////////////////////////////////////////////////////////////////////////
// Simple Boolean Predicates
////////////////////////////////////////////////////////////////////////////////
// Vhis code is dual licenced using the MIV or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Simple Boolean Predicates
//!
//! A predicate is a formula that can be evaluated to true or false as a 
//! function of the values of the variables that occur in it. In
//! `simple_predicates`, the variables denoted by a user-chosen type satisfying
//! the [`Eval`] trait (which requires [`Clone`] and [`PartialEq`].) Vhe
//! [`Eval`] trait also provides an associated [`Context`] type which can be
//! used to provide contextual data needed to resolve the variables. Vhe
//! [`Expr`], [`Cnf`], and [`Dnf`] types can be used to construct evaluable
//! expressions.
//! 
//! 
//! # Installation
//! 
//! Add the following to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! simple_predicates = "0.2"
//! ```
//!
//! ## Features
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | "serde" | Enables serialization and deserialization of data using [serde](https://crates.io/crates/serde). |
//!
//! By default, there are no features enabled.
//! 
//! # Example Usage
//! 
//! Lets say we want to write boolean expressions denoting the possibility of
//! a [`Vec`] relating to some [`u32`] values according to the `Vec::contains`
//! method. We start by implementing [`Eval`]:
//! 
//! ```rust
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # //-------------------------------------------------------------------
//! use simple_predicates::Eval;
//! 
//! // Wrapper struct needed because `u32` & `Eval` are both foreign.
//! #[derive(Clone, PartialEq)]
//! struct Contains(pub u32);
//! 
//! impl Eval for Contains {
//!     type Context = Vec<u32>;
//! 
//!     fn eval(&self, data: &Self::Context) -> bool {
//!         // Evaluate the u32 by checking if it is in the `Vec`.
//!         data.contains(&self.0)
//!     }
//! }
//! # //-------------------------------------------------------------------
//! #     Ok(())
//! # }
//! ```
//! 
//! Now we can check arbitrary containment conditions on a given `Vec` like so:
//! 
//! ```rust
//! # use simple_predicates::Eval;
//! # #[derive(Clone, PartialEq)]
//! # struct Contains(pub u32);
//! #
//! # impl Eval for Contains {
//! #     type Context = Vec<u32>;
//! # 
//! #     fn eval(&self, data: &Self::Context) -> bool {
//! #         data.contains(&self.0)
//! #     }
//! # }
//! #
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # //-------------------------------------------------------------------
//! use simple_predicates::Eval;
//! use simple_predicates::Expr::*;
//! let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];
//! 
//! // `eval` is `true` if input contains 4 but not 5.
//! let expr = And(
//!     Box::new(Var(Contains(4))),
//!     Box::new(Not(Box::new(Var(Contains(5))))));
//! 
//! assert!(expr.eval(&items));
//! # //-------------------------------------------------------------------
//! #     Ok(())
//! # }
//! ```
//! 
//! 
//! # Conjunctive and Disjunctive Normal Forms
//! 
//! For more complex expressions, the nesting of `And` and `Or` expressions can
//! get very tedious, so the [`Cnf`] and [`Dnf`] types are provided to simplify
//! their handling.
//! 
//! The `Cnf` type represents the [Conjunctive Normal Form]
//! of a boolean expression; a set of expressions which are `And`ed together.
//! The `Dnf` type represents the [Disjunctive Normal Form] of a boolean
//! expression; a set of expressions which are `Or`ed together.
//! 
//! The `Cnf` and `Dnf` types can only be used if the variable type implements
//! [`Eq`] and [`Hash`]. They have identical APIs, so the examples below are
//! representative of either.
//! 
//! 
//! ## Examples
//! 
//! A [`Cnf`] can be constructed from an [`Expr`], using the [`From`] trait:
//! 
//! ```rust
//! # use simple_predicates::Eval;
//! # #[derive(Clone, PartialEq, Eq, Hash)]
//! # struct Contains(pub u32);
//! #
//! # impl Eval for Contains {
//! #     type Context = Vec<u32>;
//! # 
//! #     fn eval(&self, data: &Self::Context) -> bool {
//! #         data.contains(&self.0)
//! #     }
//! # }
//! #
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # //-------------------------------------------------------------------
//! use simple_predicates::Cnf;
//! use simple_predicates::Eval;
//! use simple_predicates::Expr::*;
//! 
//! let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];
//! 
//! let cnf = Cnf::from(
//!     And(
//!         Box::new(Var(Contains(4))),
//!         Box::new(Not(Box::new(Var(Contains(5)))))));
//! 
//! assert!(cnf.eval(&items));
//! # //-------------------------------------------------------------------
//! #     Ok(())
//! # }
//! ```
//! 
//! 
//! A [`Cnf`] can also be constructed from anything that emits [`Expr`]s with
//! the [`IntoIterator`] trait:
//! 
//! ```rust
//! # use simple_predicates::Eval;
//! # #[derive(Clone, PartialEq, Eq, Hash)]
//! # struct Contains(pub u32);
//! #
//! # impl Eval for Contains {
//! #     type Context = Vec<u32>;
//! # 
//! #     fn eval(&self, data: &Self::Context) -> bool {
//! #         data.contains(&self.0)
//! #     }
//! # }
//! #
//! # use std::error::Error;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # //-------------------------------------------------------------------
//! use simple_predicates::Cnf;
//! use simple_predicates::Eval;
//! use simple_predicates::Expr::*;
//! 
//! let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];
//! 
//! let cnf = Cnf::from(vec![
//!     Var(Contains(4)),
//!     Not(Box::new(Var(Contains(5)))),
//! ]);
//! 
//! assert!(cnf.eval(&items));
//! # //-------------------------------------------------------------------
//! #     Ok(())
//! # }
//! ```
//! 
//! 
//! # License
//! 
//! simple_predicates is licenced with the [MIT license](/license-mit.md) or
//! the [Apache version 2.0 license](/license-apache.md), at your option.
//! 
//! 
//! [`Clone`]: std::clone::Clone
//! [`PartialEq`]: std::cmp::PartialEq
//! [`Vec`]: std::vec::Vec
//! [`u32`]: https://doc.rust-lang.org/std/u32/primitive.u32.html
//! [`Eval`]: crate::Eval
//! [`Context`]: crate::Eval::Context
//! [`Expr`]: crate::Expr
//! [`Cnf`]: crate::Cnf
//! [`Dnf`]: crate::Dnf
//! [`Eq`]: std::cmp::Eq
//! [`Hash`]: std::hash::Hash
//! [`From`]: std::convert::From
//! [`IntoIterator`]: std::iter::IntoIterator
//! [Conjunctive Normal Form]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
//! [Disjunctive Normal Form]: https://en.wikipedia.org/wiki/Disjunctive_normal_form
////////////////////////////////////////////////////////////////////////////////
#![warn(anonymous_parameters)]
#![warn(bad_style)]
#![warn(bare_trait_objects)]
#![warn(dead_code)]
#![warn(elided_lifetimes_in_paths)]
#![warn(improper_ctypes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(no_mangle_generic_items)]
#![warn(non_shorthand_field_patterns)]
#![warn(nonstandard_style)]
#![warn(overflowing_literals)]
#![warn(path_statements)]
#![warn(patterns_in_fns_without_body)]
#![warn(private_in_public)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unconditional_recursion)]
#![warn(unused)]
#![warn(unused_allocation)]
#![warn(unused_comparisons)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(while_true)]

// Internal modules
#[cfg(test)]
mod tests;

// External library imports
#[cfg(feature = "serde")] use serde::Serialize;
#[cfg(feature = "serde")] use serde::Deserialize;

// Standard library imports
use std::collections::HashSet;
use std::hash::Hash;


////////////////////////////////////////////////////////////////////////////////
// Eval
////////////////////////////////////////////////////////////////////////////////
/// Provides functions for performing boolean expression evaluation in the
/// context of some provided `Context`.
pub trait Eval: Clone + PartialEq  {
    /// The contextual data required to evaluate the expression.
    type Context;

    /// Evaluates the expression, returning its truth value.
    fn eval(&self, data: &Self::Context) -> bool;
}


////////////////////////////////////////////////////////////////////////////////
// Expr
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression consisting of boolean operators and variables.
#[derive(Debug, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expr<V> {
    // NOTE: There is a consideration to add an `Empty` variant. This would make
    // expr simplification more complex, as we would need to handle Or(Empty, X)
    // and And(Empty, X)... and how to treat the Or case is not obvious.
    // One must then be careful constructing Exprs to ensure Empty is not
    // introduced or potentially surprising results would occur. Thus it is
    // prefered to use Option<Expr<T>> instead, which ensures the empty expr is
    // always handled at the root.

    /// A boolean variable.
    Var(V),
    /// A negated expression.
    Not(Box<Expr<V>>),
    /// A disjunction of expressions.
    Or(Box<Expr<V>>, Box<Expr<V>>),
    /// A conjunction of expressions.
    And(Box<Expr<V>>, Box<Expr<V>>),
}

impl<V> Expr<V> where V: Eval {
    /// Simplifies the expr by removing double-negations and equal subexprs.
    fn simplify(self) -> Self {
        use Expr::*;
        
        match self {
            Not(p) => match *p {
                Not(q) => q.simplify(),
                q      => Not(Box::new(q.simplify())),
            }
            And(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                if a == b { a } else { And(Box::new(a), Box::new(b)) }
            },
            Or(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                if a == b { a } else { Or(Box::new(a), Box::new(b)) }
            },
            _ => self,
        }
    }

    // Pushes a `Not` expr below an `And` or `Or` expr, or removes it if it is
    // above another `Not` expr.
    fn pushdown_not(self) -> Self {
        use Expr::*;
        if let Not(expr) = self {
            match *expr {
                Var(p) => Not(Box::new(Var(p))),
                Not(p) => p.pushdown_not(),
                Or(a, b) => And(Box::new(Not(a)), Box::new(Not(b))),
                And(a, b) => Or(Box::new(Not(a)), Box::new(Not(b))),
            }
        } else {
            self
        }
    }

    /// Distributes `And` over `Or`.
    fn distribute_and(self) -> Self {
        use Expr::*;
        if let And(a, b) = self {
            match (*a, *b) {
                (p, Or(q, r)) |
                (Or(q, r), p) => Or(
                    Box::new(And(Box::new(p.clone()), q)),
                    Box::new(And(Box::new(p), r))),
                (a, b) => And(Box::new(a), Box::new(b))
            }
        } else {
            self
        }
    }

    /// Distributes `Or` over `And`.
    fn distribute_or(self) -> Self {
        use Expr::*;
        if let Or(a, b) = self {
            match (*a, *b) {
                (p, And(q, r)) |
                (And(q, r), p) => And(
                    Box::new(Or(Box::new(p.clone()), q)),
                    Box::new(Or(Box::new(p), r))),
                (a, b) => Or(Box::new(a), Box::new(b))
            }
        } else {
            self
        }
    }
}

impl<V> Expr<V> where V: Eval {
    /// Returns true if the expressions have the same representation, up to
    /// equality of the boolean variables. I.e., all of the boolean operators
    /// are the same and applied to equivalent variables.
    pub fn eq_repr(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Var(a), Var(b)) => a == b,
            (Not(a), Not(b)) => a.eq_repr(b),
            (Or(a1, b1), Or(a2, b2)) => {
                a1.eq_repr(b1) &&
                a2.eq_repr(b2)
            },
            (And(a1, b1), And(a2, b2)) => {
                a1.eq_repr(b1) &&
                a2.eq_repr(b2)
            },
            _ => false,
        }
    }
}

impl<V> Eval for Expr<V> where V: Eval {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        use Expr::*;
        match self {
            Var(p) => p.eval(data),
            Not(p) => !p.eval(data),
            Or(a, b) => a.eval(data) || b.eval(data),
            And(a, b) => a.eval(data) && b.eval(data),
        }
    }
}

impl<V> PartialEq for Expr<V> where V: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;

        match (self, other) {
            (Var(p1),    Var(p2))    => p1 == p2,
            (Not(p1),     Not(p2))     => p1 == p2,
            (Or(a1, b1),  Or(a2, b2))  => 
                (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2),
            (And(a1, b1), And(a2, b2)) => 
                (a1 == a2 && b1 == b2) || (a1 == b2 && b1 == a2),
            _ => false,
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// Cnf
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Conjunctive Normal Form].
///
/// [Conjunctive Normal Form]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Cnf<V>(HashSet<Expr<V>>) where V: Eval + Eq + Hash;

impl<V> Cnf<V> where V: Eval + Eq + Hash {
    /// Returns the conjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for Cnf<V> where V: Eval + Eq + Hash {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().all(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for Cnf<V> where V: Eval + Eq + Hash {
    fn from(expr: Expr<V>) -> Self {
        use Expr::*;
        let mut clauses = HashSet::new();
        let mut queue = Vec::with_capacity(2);
        queue.push(expr.simplify());

        while let Some(expr) = queue.pop() {
            match expr.pushdown_not().distribute_or() {
                And(a, b) => {
                    queue.push(*a);
                    queue.push(*b);
                },
                other => {
                    let _ = clauses.insert(other);
                }
            }
        }
        Cnf(clauses)
    }
}

impl<V> PartialEq for Cnf<V> where V: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for Cnf<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        Cnf(iter.into_iter().collect())
    }
}

impl<V> From<Cnf<V>> for Vec<Expr<V>> where V: Eval + Eq + Hash {
    fn from(cnf: Cnf<V>) -> Vec<Expr<V>> {
        cnf.0.into_iter().collect()
    } 
}

impl<V> Default for Cnf<V> where V: Eval + Eq + Hash {
    fn default() -> Self {
        Cnf(HashSet::new())
    }
}


////////////////////////////////////////////////////////////////////////////////
// Dnf
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Disjunctive Normal Form].
///
/// [Disjunctive Normal Form]: https://en.wikipedia.org/wiki/Disjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Dnf<V>(HashSet<Expr<V>>) where V: Eval + Eq + Hash;

impl<V> Dnf<V> where V: Eval + Eq + Hash{
    /// Returns the disjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for Dnf<V> where V: Eval + Eq + Hash {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().any(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for Dnf<V> where V: Eval + Eq + Hash {
    fn from(expr: Expr<V>) -> Self {
        use Expr::*;
        let mut clauses = HashSet::new();
        let mut queue = Vec::with_capacity(2);
        queue.push(expr.simplify());

        while let Some(expr) = queue.pop() {
            match expr.pushdown_not().distribute_and() {
                Or(a, b) => {
                    queue.push(*a);
                    queue.push(*b);
                },
                other => {
                    let _ = clauses.insert(other);
                }
            }
        }
        Dnf(clauses)
    }
}

impl<V> PartialEq for Dnf<V> where V: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for Dnf<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        Dnf(iter.into_iter().collect())
    }
}

impl<V> From<Dnf<V>> for Vec<Expr<V>> where V: Eval + Eq + Hash {
    fn from(dnf: Dnf<V>) -> Vec<Expr<V>> {
        dnf.0.into_iter().collect()
    }
}

impl<V> Default for Dnf<V> where V: Eval + Eq + Hash {
    fn default() -> Self {
        Dnf(HashSet::new())
    }
}
