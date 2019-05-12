mod ast;
use std::rc::Rc;
use std::collections::HashMap;
use crate::ast::expression::*;
use crate::ast::b_expression::*;
use crate::ast::command::*;
use crate::ast::command::C::*;

fn main() {

    testArith();
    testBoolean();
    testCommand();
    
    println!("");

}

fn testCommand() {

    println!("");

    let mut s = HashMap::new();

    // 1. Skip
    let mut s1 = evalBySmallSteps(Skip, &mut s);
    // 2. Assign
    let c1 = Assign("x", IntExp(3));
    let mut s2 = evalBySmallSteps(c1, &mut s1);
    let c1 = Assign("y", IntExp(3));
    let mut s3 = evalBySmallSteps(c1, &mut s2);

    // 3. Seq
    let c1 = SeqExp( Assign("y", IntExp(3)), Assign("z", IntExp(333)) );
    let mut s3 = evalBySmallSteps(c1, &mut s3);

    // 4. If
    let mut s = HashMap::new();
    let incX = Assign( "x", 
                        SumExp( VarExp("x"), IntExp(1) ) 
                    );
    let decX =  Assign( "x", 
                        SubExp( VarExp("x"), IntExp(1) ) 
                    );
    let cond = LtExp( VarExp("x"), IntExp(5) );

    let ifExp = IfExp(cond, incX, decX);

    let preC = Assign("x", IntExp(3));

    let program = SeqExp( preC, ifExp );
    let mut s = evalBySmallSteps(program, &mut s);
    
}

fn printB(any: B) {
    let mut s = HashMap::new();
    s.insert(Rc::new(String::from("x")), 100);
    println!("");
    print!("{:?}", any);
    println!(" => {}", evalB(any, &mut s));
}
fn printE(any: E) {
    let mut s = HashMap::new();
    s.insert(Rc::new(String::from("x")), 100);
    println!("");
    print!("{:?}", any);
    println!(" => {}", evalE(any, &mut s));
}

fn testArith() {

    let sE = SumExp(IntExp(3), IntExp(3));
    let sE2 = SumExp(  SumExp(IntExp(3), IntExp(3)),  MulExp(IntExp(3), IntExp(3)) );

    let sE3 = VarExp("x");
    // let sE4 = VarExp("y");
    printE(IntExp(3));
    printE(sE);
    printE(sE2);
    printE(sE3);
    // printE(sE4);

}

fn testBoolean() {

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
    let bE6 = LtExp(VarExp("x"), IntExp(3));
    printB(bE6);
    let bE6 = LtExp(VarExp("x"), IntExp(300));
    printB(bE6);

}
