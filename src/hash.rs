
// Internal library imports.
use crate::Eval;
use crate::Expr;

// External library imports
#[cfg(feature = "serde")] use serde::Serialize;
#[cfg(feature = "serde")] use serde::Deserialize;

// Standard library imports
use std::collections::HashSet;
use std::hash::Hash;


////////////////////////////////////////////////////////////////////////////////
// CnfHashSet
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Conjunctive Normal Form], stored as a `HashSet`.
///
/// [Conjunctive Normal Form]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CnfHashSet<V>(HashSet<Expr<V>>) where V: Eval + Eq + Hash;

impl<V> CnfHashSet<V> where V: Eval + Eq + Hash {
    /// Returns the conjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for CnfHashSet<V> where V: Eval + Eq + Hash {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().all(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for CnfHashSet<V> where V: Eval + Eq + Hash {
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
        CnfHashSet(clauses)
    }
}

impl<V> PartialEq for CnfHashSet<V> where V: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for CnfHashSet<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        CnfHashSet(iter.into_iter().collect())
    }
}

impl<V> From<CnfHashSet<V>> for Vec<Expr<V>> where V: Eval + Eq + Hash {
    fn from(cnf: CnfHashSet<V>) -> Vec<Expr<V>> {
        cnf.0.into_iter().collect()
    } 
}

impl<V> Default for CnfHashSet<V> where V: Eval + Eq + Hash {
    fn default() -> Self {
        CnfHashSet(HashSet::new())
    }
}


////////////////////////////////////////////////////////////////////////////////
// DnfHashSet
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Disjunctive Normal Form], stored as a `HashSet`.
///
/// [Disjunctive Normal Form]: https://en.wikipedia.org/wiki/Disjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DnfHashSet<V>(HashSet<Expr<V>>) where V: Eval + Eq + Hash;

impl<V> DnfHashSet<V> where V: Eval + Eq + Hash{
    /// Returns the disjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for DnfHashSet<V> where V: Eval + Eq + Hash {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().any(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for DnfHashSet<V> where V: Eval + Eq + Hash {
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
        DnfHashSet(clauses)
    }
}

impl<V> PartialEq for DnfHashSet<V> where V: Eval + Eq + Hash {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for DnfHashSet<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq + Hash
{
    fn from(iter: I) -> Self {
        DnfHashSet(iter.into_iter().collect())
    }
}

impl<V> From<DnfHashSet<V>> for Vec<Expr<V>> where V: Eval + Eq + Hash {
    fn from(dnf: DnfHashSet<V>) -> Vec<Expr<V>> {
        dnf.0.into_iter().collect()
    }
}

impl<V> Default for DnfHashSet<V> where V: Eval + Eq + Hash {
    fn default() -> Self {
        DnfHashSet(HashSet::new())
    }
}
