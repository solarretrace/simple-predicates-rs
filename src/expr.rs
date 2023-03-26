

// External library imports
#[cfg(feature = "serde")] use serde::Serialize;
#[cfg(feature = "serde")] use serde::Deserialize;

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
    pub (in crate) fn simplify(self) -> Self {
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
    pub (in crate) fn pushdown_not(self) -> Self {
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
    pub (in crate) fn distribute_and(self) -> Self {
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
    pub (in crate) fn distribute_or(self) -> Self {
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

