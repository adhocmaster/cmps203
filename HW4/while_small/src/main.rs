mod ast;
use std::collections::HashMap;
use crate::ast::expression::*;
use crate::ast::b_expression::*;
use crate::ast::command::*;
use crate::ast::command::C::*;

fn printB(any: B) {
    println!("");
    print!("{:?}", any);
    println!(" => {}", evalB(any));
}
fn printE(any: E) {
    println!("");
    print!("{:?}", any);
    println!(" => {}", evalE(any));
}
fn main() {

    let sE = SumExp(IntExp(3), IntExp(3));
    let sE2 = SumExp(  SumExp(IntExp(3), IntExp(3)),  MulExp(IntExp(3), IntExp(3)) );
    printE(IntExp(3));
    printE(sE);
    printE(sE2);

    let bE1 = BoolExp(true);
    let bE2 = AndExp(BoolExp(true), BoolExp(true));
    let bE3 = AndExp(BoolExp(true), BoolExp(false));
    let bE4 = OrExp(BoolExp(true), BoolExp(false));
    let bE5 = EQExp(IntExp(3), IntExp(3));
    let bE6 = LtExp(IntExp(3), IntExp(3));
    printB(bE1);
    printB(bE2);
    printB(bE3);
    printB(bE4);
    printB(bE5);
    printB(bE6);

    
    println!("");

    let s = HashMap::new();

    let s2 = evalC(Skip, s);

}
