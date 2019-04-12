package com.adhocmaster.cmps203updated.arith2

object Interpreter {

  def eval( node: AST.Node ): Int = {

    if ( node == AST.DummyNode )
      throw new Exception( "Incomplete AST" )

    node match {

      case _: AST.NumeralNode => {
        return node.asInstanceOf[AST.NumeralNode].value
      }
      case _: AST.BinaryOperatorNode => {
        evalAsBinary( node )
      }
      case _: AST.UnaryOperatorNode => {
        evalAsUnary( node )
      }
    }

  }

  def evalAsBinary( node: AST.Node ): Int = {

    var binaryNode = node.asInstanceOf[AST.BinaryOperatorNode]
    val leftVal = eval( binaryNode.left.getOrElse( AST.DummyNode ) )
    val rightVal = eval( binaryNode.right.getOrElse( AST.DummyNode ) )

    return binaryNode.value match {

      case AST.Operations.Add => leftVal + rightVal
      case AST.Operations.Mul => leftVal * rightVal
      case AST.Operations.Mod => leftVal % rightVal

    }
  }

  def evalAsUnary( node: AST.Node ): Int = {

    var unaryNode = node.asInstanceOf[AST.UnaryOperatorNode]
    val operand = eval( unaryNode.operand.getOrElse( AST.DummyNode ) )

    return unaryNode.value match {

      case AST.Operations.Inc => return operand + 1

    }
  }
}
