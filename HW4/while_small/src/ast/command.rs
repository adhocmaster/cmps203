/*
data C = Skip Bool
        | Assign Var E
        | Seq C C
        | If B C C
        | While B C
        | Inc Var
        */
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
    Ass(E, E)
}

pub fn printState(s: &HashMap<&str, i32>) {

    print!("{{");

    for (k, v) in s.iter() {
        print!( "{}:{}, ", k, v );
    }

    print!("}}");

}
pub fn evalC(c: C, s: HashMap<&str, i32>) -> HashMap<&str, i32> {

    let s2 = match c {

        Skip => { 
            print!("<skip, "); 
            printState(&s);
            println!(">");
            s 
            },
        
        Ass(var, val) => {

            print!("<skip, "); 
            printState(&s);
            println!(">");
            s 
            }

    };

    s2

}