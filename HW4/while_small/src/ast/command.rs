/*
data C = Skip Bool
        | Assign Var E
        | Seq C C
        | If B C C
        | While B C
        | Inc Var
        */
use std::rc::Rc;
use std::collections::HashMap;
use super::expression::E;
use super::expression::evalE;
use super::b_expression::B;
use super::b_expression::evalB;

#[allow(dead_code)]

use C::*;
#[derive(Debug)]
pub enum C {
    Skip,
    Ass(Rc<String>, E)
}

pub fn printState(s: &HashMap<Rc<String>, i32>) {

    print!("{{");

    let mut comma = "";

    for (k, v) in s.iter() {
        print!( "{}{}:{}", comma, k, v );
        comma = ", ";
    }

    print!("}}");

}
pub fn evalC(c: C, s: &mut HashMap<Rc<String>, i32>) -> &mut HashMap<Rc<String>, i32> {

    let s2 = match c {

        Skip => { 
            print!("<skip, "); 
            printState(&s);
            println!(">");
            s 
            },
        
        Ass(var, e) => {

            let val = evalE(e);
            print!("<Assign {} {}, ", var, val); 
            s.insert(var, val);
            printState(&s);
            println!(">");
            s 
            }

    };

    s2

}

pub fn Assign(var: &str, e: E) -> C {

    Ass(Rc::new( String::from(var) ), e)

}