////////////////////////////////////////////////////////////////////////////////
// Simple Boolean Predicates
////////////////////////////////////////////////////////////////////////////////
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Tests module
////////////////////////////////////////////////////////////////////////////////

use crate::*;


impl Eval for u32 {
    type Context = Vec<u32>;
    fn eval(&self, data: &Self::Context) -> bool {
        data.contains(self)
    }
}

#[test]
fn simple_bool() {
    use Expr::*;
    let items: Vec<u32> = vec![1, 2, 4, 5, 7, 9, 10];

    let expr = Var(4);
    assert!(expr.eval(&items));

    let expr = Var(3);
    assert!(!expr.eval(&items));
}

#[test]
fn simple_not() {
    use Expr::*;
    let items: Vec<u32> = vec![1, 2, 4, 5, 7, 9, 10];

    let expr = Not(Box::new(Var(4)));
    assert!(!expr.eval(&items));

    let expr = Not(Box::new(Var(3)));
    assert!(expr.eval(&items));
}

#[test]
fn simple_or() {
    use Expr::*;
    let items: Vec<u32> = vec![1, 2, 4, 5, 7, 9, 10];

    let expr = Or(Box::new(Var(4)), Box::new(Var(5)));
    assert!(expr.eval(&items));

    let expr = Or(Box::new(Var(3)), Box::new(Var(4)));
    assert!(expr.eval(&items));


    let expr = Or(Box::new(Var(3)), Box::new(Var(6)));
    assert!(!expr.eval(&items));
}

#[test]
fn simple_and() {
    use Expr::*;
    let items: Vec<u32> = vec![1, 2, 4, 5, 7, 9, 10];

    let expr = And(Box::new(Var(4)), Box::new(Var(5)));
    assert!(expr.eval(&items));

    let expr = And(Box::new(Var(3)), Box::new(Var(4)));
    assert!(!expr.eval(&items));

    let expr = And(Box::new(Var(3)), Box::new(Var(6)));
    assert!(!expr.eval(&items));
}


#[test]
fn three_level_cnf() {
    use Expr::*;

    let expr = And(
        Box::new(Or(
            Box::new(And(
                Box::new(Var(1)),
                Box::new(Var(2)))),
            Box::new(And(
                Box::new(Var(3)),
                Box::new(Var(4)))))),
        Box::new(And(
            Box::new(Or(
                Box::new(Var(5)),
                Box::new(Var(6)))),
            Box::new(Or(
                Box::new(Var(7)),
                Box::new(Var(8)))))));

    let res = Cnf::from(expr);
    let cnf = Cnf::from(vec![
        Or(
            Box::new(Var(7)),
            Box::new(Var(8))),
        Or(
            Box::new(Var(5)),
            Box::new(Var(6))),
        Or(
            Box::new(Var(4)),
            Box::new(Var(2))),
        Or(
            Box::new(Var(4)),
            Box::new(Var(1))),
        Or(
            Box::new(Var(3)),
            Box::new(Var(2))),
        Or(
            Box::new(Var(3)),
            Box::new(Var(1))),
    ]);

    // println!("{:?}", res);
    // println!("{:?}", cnf);

    assert_eq!(res, cnf);

}


#[test]
fn three_level_dnf() {
    use Expr::*;

    let expr = And(
        Box::new(Or(
            Box::new(And(
                Box::new(Var(1)),
                Box::new(Var(2)))),
            Box::new(And(
                Box::new(Var(3)),
                Box::new(Var(4)))))),
        Box::new(And(
            Box::new(Or(
                Box::new(Var(5)),
                Box::new(Var(6)))),
            Box::new(Or(
                Box::new(Var(7)),
                Box::new(Var(8)))))));

    let res = Dnf::from(expr);
    let dnf = Dnf::from(vec![
        And(
            Box::new(And(
                Box::new(Or(
                    Box::new(Var(5)),
                    Box::new(Var(6)))),
                Box::new(Or(
                    Box::new(Var(7)),
                    Box::new(Var(8)))))), 
            Box::new(And(
                Box::new(Var(3)),
                Box::new(Var(4))))),
        And(
            Box::new(And(
                Box::new(Or(
                    Box::new(Var(5)),
                    Box::new(Var(6)))),
                Box::new(Or(
                    Box::new(Var(7)),
                    Box::new(Var(8)))))),
            Box::new(And(
                Box::new(Var(1)),
                Box::new(Var(2))))),
    ]);

    // println!("{:?}", res);
    // println!("{:?}", dnf);

    assert_eq!(res, dnf);
}


#[cfg(not(feature = "serde"))]
#[test]
fn serialize_tests() {
    panic!("Enable \"serde\" (--all-features) feature to run serde tests.")
}

#[cfg(feature = "serde")]
#[test]
fn serialize_expr_ron() {
    use Expr::*;
    let expr = And(
        Box::new(Or(
            Box::new(Not(Box::new(Var(1)))),
            Box::new(Var(2)))),
        Box::new(Or(
            Box::new(Not(Box::new(Var(3)))),
            Box::new(Var(4)))));

    let s = ron::ser::to_string(&expr).unwrap();
    assert_eq!(s, "And(Or(Not(Var(1)),Var(2)),Or(Not(Var(3)),Var(4)))");
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_expr_ron() {
    use Expr::*;
    let expr = And(
        Box::new(Or(
            Box::new(Not(Box::new(Var(1)))),
            Box::new(Var(2)))),
        Box::new(Or(
            Box::new(Not(Box::new(Var(3)))),
            Box::new(Var(4)))));
    let s = "And(Or(Not(Var(1)),Var(2)),Or(Not(Var(3)),Var(4)))";

    use ron::de::Deserializer;
    let mut d = Deserializer::from_str(&s).unwrap();
    let res = Expr::deserialize(&mut d).unwrap();
    d.end().unwrap();

    assert_eq!(res, expr);
}
