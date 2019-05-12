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
    Ass(Rc<String>, E),
    Seq(Box<C>, Box<C>),
    Nil
}

pub fn getStateString(s: &HashMap<Rc<String>, i32>) -> String {

    let mut message = String::new();

    message.push_str( &format!("{{") );

    let mut comma = "";

    for (k, v) in s.iter() {
        message.push_str( &format!( "{}{}:{}", comma, k, v ) );
        comma = ", ";
    }

    message.push_str( &format!("}}") );
    message

}
pub fn printState(s: &HashMap<Rc<String>, i32>) {

    print!( "{}", getStateString(s) );

}

pub fn printCandS(c: &C, s: &HashMap<Rc<String>, i32>) {

    print!("<{:?}, {}>", *c, getStateString(s)); 
    println!("");

}


pub fn evalSmallStep(c: C, s: &mut HashMap<Rc<String>, i32>) -> (C, &mut HashMap<Rc<String>, i32>) {

    let (c2, mut s2) = match c {

        Nil => {
            // print!("<skip, "); 
            // printState(&s);
            // println!(">");
            (Nil, s) 
            },

        Skip => { 
            // print!("<skip, "); 
            // printState(&s);
            // println!(">");
            (Nil, s) 
            },
        
        Ass(var, e) => {

            // let message = String::new();
            let val = evalE(e);
            // message.push_str( format!("Assign {} {}, ", var, val) ); 
            s.insert(var, val);
            // printState(&s);
            // message.push_str( format!(">") );
            (Skip, s) // Fix this
            },
        Seq(c1, c2) => {

            let (c3, mut s1) = evalSmallStep(*c1, s);

            match c3 {
                Nil => (*c2, s1),
                _ => ( SeqExp(c3, *c2), s1 )
            }

        }

    };

    (c2, s2)

}

pub fn evalBySmallSteps(c: C, s: &mut HashMap<Rc<String>, i32>) -> &mut HashMap<Rc<String>, i32> {

    println!("*******************Running new program*********************");

    let mut optional = Some(c);
    let mut counter = 0;

    loop {

        match optional {

            Some(Nil) => { 
                println!("*******************Ending new program*********************");
                break s;
                },
            Some(c2) => {
                
                counter += 1;
                print!("Step #{}: ", counter);
                printCandS(&c2, s);
                let (c3, s) = evalSmallStep(c2, s);
                optional = Some(c3);

            }
            _ => { 
                println!("*******************Ending new program*********************");
                break s;
                },

        }

    }
    

}

pub fn Assign(var: &str, e: E) -> C {

    Ass(Rc::new( String::from(var) ), e)

}

pub fn SeqExp(c1: C, c2: C) -> C {

    Seq( Box::new(c1), Box::new(c2) )

}