
// Internal library imports.
use crate::Eval;
use crate::Expr;

// External library imports
#[cfg(feature = "serde")] use serde::Serialize;
#[cfg(feature = "serde")] use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
// CnfVec
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Conjunctive Normal Form], stored as a `Vec`.
///
/// [Conjunctive Normal Form]: https://en.wikipedia.org/wiki/Conjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CnfVec<V>(Vec<Expr<V>>) where V: Eval + Eq;

impl<V> CnfVec<V> where V: Eval + Eq {
    /// Returns the conjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for CnfVec<V> where V: Eval + Eq {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().all(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for CnfVec<V> where V: Eval + Eq {
    fn from(expr: Expr<V>) -> Self {
        use Expr::*;
        let mut clauses = Vec::new();
        let mut queue = Vec::with_capacity(2);
        queue.push(expr.simplify());

        while let Some(expr) = queue.pop() {
            match expr.pushdown_not().distribute_or() {
                And(a, b) => {
                    queue.push(*a);
                    queue.push(*b);
                },
                other => {
                    let _ = clauses.push(other);
                }
            }
        }
        CnfVec(clauses)
    }
}

impl<V> PartialEq for CnfVec<V> where V: Eval + Eq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for CnfVec<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq
{
    fn from(iter: I) -> Self {
        CnfVec(iter.into_iter().collect())
    }
}

impl<V> From<CnfVec<V>> for Vec<Expr<V>> where V: Eval + Eq {
    fn from(cnf: CnfVec<V>) -> Vec<Expr<V>> {
        cnf.0.into_iter().collect()
    } 
}

impl<V> Default for CnfVec<V> where V: Eval + Eq {
    fn default() -> Self {
        CnfVec(Vec::new())
    }
}


////////////////////////////////////////////////////////////////////////////////
// DnfVec
////////////////////////////////////////////////////////////////////////////////
/// A boolean expression in [Disjunctive Normal Form], stored as a `Vec`.
///
/// [Disjunctive Normal Form]: https://en.wikipedia.org/wiki/Disjunctive_normal_form
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DnfVec<V>(Vec<Expr<V>>) where V: Eval + Eq;

impl<V> DnfVec<V> where V: Eval + Eq{
    /// Returns the disjunctive clauses as elements of a `Vec`.
    pub fn into_vec(self) -> Vec<Expr<V>> {
        self.0.into_iter().collect()
    }

    /// Returns true if the boolean expression contains no terms.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<V> Eval for DnfVec<V> where V: Eval + Eq {
    type Context = V::Context;

    fn eval(&self, data: &Self::Context) -> bool {
        self.0.iter().any(|expr| expr.eval(data))
    }
}

impl<V> From<Expr<V>> for DnfVec<V> where V: Eval + Eq {
    fn from(expr: Expr<V>) -> Self {
        use Expr::*;
        let mut clauses = Vec::new();
        let mut queue = Vec::with_capacity(2);
        queue.push(expr.simplify());

        while let Some(expr) = queue.pop() {
            match expr.pushdown_not().distribute_and() {
                Or(a, b) => {
                    queue.push(*a);
                    queue.push(*b);
                },
                other => {
                    let _ = clauses.push(other);
                }
            }
        }
        DnfVec(clauses)
    }
}

impl<V> PartialEq for DnfVec<V> where V: Eval + Eq {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<I, V> From<I> for DnfVec<V> where
    I: IntoIterator<Item=Expr<V>>,
    V: Eval + Eq
{
    fn from(iter: I) -> Self {
        DnfVec(iter.into_iter().collect())
    }
}

impl<V> From<DnfVec<V>> for Vec<Expr<V>> where V: Eval + Eq {
    fn from(dnf: DnfVec<V>) -> Vec<Expr<V>> {
        dnf.0.into_iter().collect()
    }
}

impl<V> Default for DnfVec<V> where V: Eval + Eq {
    fn default() -> Self {
        DnfVec(Vec::new())
    }
}
