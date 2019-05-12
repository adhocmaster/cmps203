
use std::collections::HashMap;
use std::rc::Rc;
#[allow(dead_code)]
use E::*;

#[derive(Debug)]
#[derive(Clone)]
pub enum E {

    IntE(i32),
    VarE(Rc<String>),
    SumE(Box<E>, Box<E>),
    SubE(Box<E>, Box<E>),
    MulE(Box<E>, Box<E>)

}


pub fn evalE(e: E, s: &mut HashMap<Rc<String>, i32>) -> i32 {

    let val = match e {

        IntE(n) => n,
        VarE(var) => {
            *(s.get(&var).unwrap())
        },
        SumE(e1, e2) => evalE(*e1, s) + evalE(*e2, s),
        SubE(e1, e2) => evalE(*e1, s) - evalE(*e2, s),
        MulE(e1, e2) => evalE(*e1, s) * evalE(*e2, s)

    };

    val

}

pub fn IntExp(n:i32) -> E {

    IntE(n)

}

pub fn VarExp(var: &str) -> E {

    VarE(Rc::new( String::from(var) ))

}
pub fn SumExp(e1: E, e2:E) -> E {

    SumE(Box::new(e1), Box::new(e2) )

}
pub fn SubExp(e1: E, e2:E) -> E {

    SubE(Box::new(e1), Box::new(e2) )

}
pub fn MulExp(e1: E, e2:E) -> E {

    MulE(Box::new(e1), Box::new(e2) )

}
