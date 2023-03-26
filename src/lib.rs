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
//! [`Expr`], [`CnfHashSet`], and [`DnfHashSet`] types can be used to construct evaluable
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
//! get very tedious, so the [`CnfHashSet`] and [`DnfHashSet`] types are provided to simplify
//! their handling.
//! 
//! The `CnfHashSet` type represents the [Conjunctive Normal Form]
//! of a boolean expression; a set of expressions which are `And`ed together.
//! The `DnfHashSet` type represents the [Disjunctive Normal Form] of a boolean
//! expression; a set of expressions which are `Or`ed together.
//! 
//! The `CnfHashSet` and `DnfHashSet` types can only be used if the variable type implements
//! [`Eq`] and [`Hash`]. They have identical APIs, so the examples below are
//! representative of either.
//! 
//! 
//! ## Examples
//! 
//! A [`CnfHashSet`] can be constructed from an [`Expr`], using the [`From`] trait:
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
//! use simple_predicates::CnfHashSet;
//! use simple_predicates::Eval;
//! use simple_predicates::Expr::*;
//! 
//! let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];
//! 
//! let cnf = CnfHashSet::from(
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
//! A [`CnfHashSet`] can also be constructed from anything that emits [`Expr`]s with
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
//! use simple_predicates::CnfHashSet;
//! use simple_predicates::Eval;
//! use simple_predicates::Expr::*;
//! 
//! let items: Vec<u32> = vec![1, 2, 4, 7, 9, 10];
//! 
//! let cnf = CnfHashSet::from(vec![
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
//! [`CnfHashSet`]: crate::CnfHashSet
//! [`DnfHashSet`]: crate::DnfHashSet
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
mod expr;
mod hash;
mod vec;
#[cfg(test)]
mod tests;


pub use crate::expr::*;
pub use crate::hash::*;
pub use crate::vec::*;
