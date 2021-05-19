////////////////////////////////////////////////////////////////////////////////
// Simple Boolean Predicates
////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Simple Boolean Predicates
////////////////////////////////////////////////////////////////////////////////
#![warn(anonymous_parameters)]
#![warn(bad_style)]
#![warn(bare_trait_objects)]
#![warn(const_err)]
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

// External library imports.
#[cfg(feature = "serde")] use serde::Serialize;
#[cfg(feature = "serde")] use serde::Deserialize;
#[cfg(test)]
mod tests;


use std::hash::Hash;
use std::collections::HashSet;

////////////////////////////////////////////////////////////////////////////////
// Eval
////////////////////////////////////////////////////////////////////////////////
/// Provides functions for performing boolean expression evaluation in the
/// context of some provided `Data`.
pub trait Eval: Clone + PartialEq  {
    /// The contextual data used to evaluate the expression.
    type Data;

    /// Evaluates the expression, returning its truth value.
    fn eval(&self, data: &Self::Data) -> bool;
}



////////////////////////////////////////////////////////////////////////////////
// Expr
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression consisting of boolean operators and literals.
#[derive(Debug, Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expr<T> {
    /// A boolean-evaluable literal.
    Bool(T),
    /// A negated expression.
    Not(Box<Expr<T>>),
    /// A disjunction of expressions.
    Or(Box<Expr<T>>, Box<Expr<T>>),
    /// A conjunction of expressions.
    And(Box<Expr<T>>, Box<Expr<T>>),
}

impl<T> Expr<T> where T: Eval {
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

    fn pushdown_not(self) -> Self {
        use Expr::*;
        if let Not(expr) = self {
            match *expr {
                Bool(p) => Not(Box::new(Bool(p))),
                Not(p) => p.pushdown_not(),
                Or(a, b) => And(Box::new(Not(a)), Box::new(Not(b))),
                And(a, b) => Or(Box::new(Not(a)), Box::new(Not(b))),
            }
        } else {
            self
        }
    }

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

impl<T> Expr<T> where T: Eval {
    /// Returns true if the expressions have the same representation, up to
    /// equality of the boolean literals. I.e., all of the boolean operators are
    /// are the same and applied to equivalent literals.
    pub fn eq_repr(&self, other: &Self) -> bool {
        use Expr::*;
        match (self, other) {
            (Bool(a), Bool(b)) => a == b,
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

impl<T> Eval for Expr<T> where T: Eval {
    type Data = T::Data;

    fn eval(&self, data: &Self::Data) -> bool {
        use Expr::*;
        match self {
            Bool(p) => p.eval(data),
            Not(p) => !p.eval(data),
            Or(a, b) => a.eval(data) || b.eval(data),
            And(a, b) => a.eval(data) && b.eval(data),
        }
    }
}


impl<T> PartialEq for Expr<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        use Expr::*;

        match (self, other) {
            (Bool(p1),    Bool(p2))    => p1 == p2,
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
/// A boolean expression in Conjunctive Normal Form.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Cnf<T>(HashSet<Expr<T>>) where T: Eval + Eq + Hash;

impl<T> Cnf<T> where T: Eval + Eq + Hash {
    /// Returns the conjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<T>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Eval for Cnf<T> where T: Eval + Eq + Hash {
    type Data = T::Data;

    fn eval(&self, data: &Self::Data) -> bool {
        self.0.iter().all(|expr| expr.eval(data))
    }
}

impl<T> From<Expr<T>> for Cnf<T> where T: Eval + Eq + Hash {
    fn from(expr: Expr<T>) -> Self {
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


impl<T> PartialEq for Cnf<T> where T: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, T> From<I> for Cnf<T> where
    I: IntoIterator<Item=Expr<T>>,
    T: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        Cnf(iter.into_iter().collect())
    }
}

impl<T> From<Cnf<T>> for Vec<Expr<T>> where T: Eval + Eq + Hash {
    fn from(cnf: Cnf<T>) -> Vec<Expr<T>> {
        cnf.0.into_iter().collect()
    } 
}

////////////////////////////////////////////////////////////////////////////////
// Dnf
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in Disjunctive Normal Form.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Dnf<T>(HashSet<Expr<T>>) where T: Eval + Eq + Hash;

impl<T> Dnf<T> where T: Eval + Eq + Hash{
    /// Returns the disjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<T>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<T> Eval for Dnf<T> where T: Eval + Eq + Hash {
    type Data = T::Data;

    fn eval(&self, data: &Self::Data) -> bool {
        self.0.iter().any(|expr| expr.eval(data))
    }
}

impl<T> From<Expr<T>> for Dnf<T> where T: Eval + Eq + Hash {
    fn from(expr: Expr<T>) -> Self {
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


impl<T> PartialEq for Dnf<T> where T: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, T> From<I> for Dnf<T> where
    I: IntoIterator<Item=Expr<T>>,
    T: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        Dnf(iter.into_iter().collect())
    }
}

impl<T> From<Dnf<T>> for Vec<Expr<T>> where T: Eval + Eq + Hash {
    fn from(dnf: Dnf<T>) -> Vec<Expr<T>> {
        dnf.0.into_iter().collect()
    }
}
