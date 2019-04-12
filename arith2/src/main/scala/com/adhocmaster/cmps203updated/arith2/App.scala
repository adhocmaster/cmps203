package com.adhocmaster.cmps203updated.arith2

/**
 * @author ${user.name}
 */
object App {

  def main( args: Array[String] ) {
    println( "Hello World!" )

    Test1

  }

  def Test1() {

    val ast = new AST( "5" )
    val root = new ast.NumeralNode( 5 )
    ast.setRoot( root )
    print( ast.eval )

  }

}
