package com.adhocmaster.cmps203updated.arith2

object AST {

  object Operations extends Enumeration {

    val Add, Mul, Mod = Value

  }

  class Node( isTerminalIn: Boolean ) {

    var isTerminal = isTerminalIn

  }

  object DummyNode extends Node( true ) {}

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

}
