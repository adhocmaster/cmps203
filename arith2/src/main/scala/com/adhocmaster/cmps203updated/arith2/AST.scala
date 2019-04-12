package com.adhocmaster.cmps203updated.arith2

class AST( exp: String ) {

  var name = exp
  var root: Option[Node] = None

  def setRoot( node: Node ) = { this.root = Some( node ) }

  class Node( isTerminalIn: Boolean ) {

    var isTerminal = isTerminalIn

  }
  class NumeralNode( valueIn: Int ) extends Node( true ) {

    var value = valueIn

  }

  class BinaryOperatorNode( opType: Operations.Value ) extends Node( false ) {

    var value = opType
    var left: Option[Node] = None
    var right: Option[Node] = None

    def setLeft( node: Node ) = { this.left = Some( node ) }
    def setRight( node: Node ) = { this.right = Some( node ) }

  }

  def eval(): Int = {

    if ( root == None )
      throw new Exception( "root node not set" )

    return eval( root.get )

  }

  def eval( node: Node ): Int = {

    if ( node == None )
      throw new Exception( "Incomplete AST" )

    if ( node.isTerminal == true )
      return node.asInstanceOf[NumeralNode].value

    var binaryNode = node.asInstanceOf[BinaryOperatorNode]
    val leftVal = eval( binaryNode.left.get )
    val rightVal = eval( binaryNode.right.get )

    return binaryNode.value match {

      case Operations.Add => leftVal + rightVal
      case Operations.Mul => leftVal * rightVal

    }

  }

}
