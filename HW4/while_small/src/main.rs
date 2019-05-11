mod ast;
use crate::ast::expression::expression::*;
fn main() {
    println!("IntExp(3) => {}", evalE(IntExp(3)));

    let sE = SumExp(IntExp(3), IntExp(3));
    let sE2 = SumExp(  SumExp(IntExp(3), IntExp(3)),  MulExp(IntExp(3), IntExp(3)) );
    println!("SumExp(IntExp(3), IntExp(3)) => {}", evalE(sE) );
    println!("sE2 = {:?}", sE2 );
    println!("sE2 => {}", evalE(sE2) );
}
