package com.adhocmaster.cmps203updated.arith2

/**
 * @author ${Golam Md Muktadir}
 */
object App {

  def main( args: Array[String] ) {
    println( "Arith AST and Interpreter in Scala by Golam Md Muktadir" )

    TestNumeral
    TestBinaryNoOperand
    TestBinaryLeftOperand
    TestBinaryRightOperand
    TestAdd
    TestMul
    TestExpressionFromSlides
    TestExpressionFromSlidesMirrored

  }

  def TestNumeral() {

    println( "\nTest: TestNumeral" )

    val root = new AST.NumeralNode( 5 )
    println( "Result: " + Interpreter.eval( root ) )

  }

  def TestBinaryNoOperand() {

    println( "\nTest: TestBinaryNoOperand with expression: +" )

    val root = new AST.BinaryOperatorNode( AST.Operations.Add )

    try {

      println( "Result: " + Interpreter.eval( root ) )

    } catch {

      case e: Exception => println( "Error: " + e.getMessage )

    }

  }

  def TestBinaryLeftOperand() {

    println( "\nTest: TestBinaryLeftOperand with expression: 5 +" )

    val root = new AST.BinaryOperatorNode( AST.Operations.Add )
    root.setLeft( new AST.NumeralNode( 5 ) )

    try {

      println( "Result: " + Interpreter.eval( root ) )

    } catch {

      case e: Exception => println( "Error: " + e.getMessage )

    }

  }

  def TestBinaryRightOperand() {

    println( "\nTest: TestBinaryRightOperand with expression: * 7" )

    val root = new AST.BinaryOperatorNode( AST.Operations.Mul )
    root.setRight( new AST.NumeralNode( 7 ) )
    try {

      println( "Result: " + Interpreter.eval( root ) )

    } catch {

      case e: Exception => println( "Error: " + e.getMessage )

    }

  }

  def TestAdd() {

    println( "\nTest: TestAdd with expression: 5 + 7" )

    val root = new AST.BinaryOperatorNode( AST.Operations.Add )

    root.setLeft( new AST.NumeralNode( 5 ) )
    root.setRight( new AST.NumeralNode( 7 ) )

    println( "Result: " + Interpreter.eval( root ) )

    assert( Interpreter.eval( root ) == 12 )

  }

  def TestMul() {

    println( "\nTest: TestAdd with expression: 5 + 7" )

    val root = new AST.BinaryOperatorNode( AST.Operations.Mul )

    root.setLeft( new AST.NumeralNode( 5 ) )
    root.setRight( new AST.NumeralNode( 7 ) )

    println( "Result: " + Interpreter.eval( root ) )

    assert( Interpreter.eval( root ) == 35 )

  }

  def TestExpressionFromSlides() {

    println( "\nTest: TestExpressionFromSlides with expression: (3 + 5) * 2 " )

    val root = new AST.BinaryOperatorNode( AST.Operations.Mul )

    var sum = new AST.BinaryOperatorNode( AST.Operations.Add )

    sum.setLeft( new AST.NumeralNode( 3 ) )
    sum.setRight( new AST.NumeralNode( 5 ) )

    root.setLeft( sum )
    root.setRight( new AST.NumeralNode( 2 ) )

    println( "Result: " + Interpreter.eval( root ) )

    assert( Interpreter.eval( root ) == 16 )

  }

  def TestExpressionFromSlidesMirrored() {

    println( "\nTest: TestExpressionFromSlidesAugmented with expression: ((3 + 5) * 2) + ((3 + 5) * 2) " )

    val mul = new AST.BinaryOperatorNode( AST.Operations.Mul )

    var sum = new AST.BinaryOperatorNode( AST.Operations.Add )

    sum.setLeft( new AST.NumeralNode( 3 ) )
    sum.setRight( new AST.NumeralNode( 5 ) )

    mul.setLeft( sum )
    mul.setRight( new AST.NumeralNode( 2 ) )

    var root = new AST.BinaryOperatorNode( AST.Operations.Add )
    root.setLeft( mul )
    root.setRight( mul )

    println( "Result: " + Interpreter.eval( root ) )

    assert( Interpreter.eval( root ) == 32 )

  }

}
