
use std::collections::HashMap;
use std::rc::Rc;
use super::expression::E;
use super::expression::evalE;
#[allow(dead_code)]

use B::*;

#[derive(Debug)]

pub enum B {
    BoolE(bool),
    EQB(Box<E>, Box<E>),
    LtB(Box<E>, Box<E>),
    NotB(Box<B>),
    AndB(Box<B>, Box<B>),
    OrB(Box<B>, Box<B>)
}

pub fn evalB(e: B, s: &mut HashMap<Rc<String>, i32>) -> bool {

    let val = match e {

        BoolE(b) => b,
        EQB(e1, e2) => evalE(*e1, s) == evalE(*e2, s),
        LtB(e1, e2) => evalE(*e1, s) < evalE(*e2, s),
        NotB(b) => !evalB(*b, s),
        AndB(b1, b2) => evalB(*b1, s) && evalB(*b2, s),
        OrB(b1, b2) => evalB(*b1, s) || evalB(*b2, s),

    };

    val

}


pub fn BoolExp(b: bool) -> B {

    BoolE(b)

}

pub fn EQExp(e1: E, e2:E) -> B {

    EQB( Box::new(e1), Box::new(e2) )

}
pub fn LtExp(e1: E, e2:E) -> B {

    LtB( Box::new(e1), Box::new(e2) )

}
pub fn NotExp(b: B) -> B {

    NotB(Box::new(b))

}
pub fn AndExp(b1: B, b2: B) -> B {

    AndB( Box::new(b1), Box::new(b2) )

}
pub fn OrExp(b1: B, b2: B) -> B {

    OrB( Box::new(b1), Box::new(b2) )

}
