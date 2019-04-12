package com.adhocmaster.cmps203updated.arith2

object Interpreter {

  def eval( node: AST.Node ): Int = {

    if ( node == AST.DummyNode )
      throw new Exception( "Incomplete AST" )

    if ( node.isTerminal == true )
      return node.asInstanceOf[AST.NumeralNode].value

    var binaryNode = node.asInstanceOf[AST.BinaryOperatorNode]
    val leftVal = eval( binaryNode.left.getOrElse( AST.DummyNode ) )
    val rightVal = eval( binaryNode.right.getOrElse( AST.DummyNode ) )

    return binaryNode.value match {

      case AST.Operations.Add => leftVal + rightVal
      case AST.Operations.Mul => leftVal * rightVal

    }

  }
}
