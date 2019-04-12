package com.adhocmaster.cmps203updated.arith2

object AST {

  object Operations extends Enumeration {

    val Add, Mul, Mod, Inc = Value

  }

  class Node() {

  }

  object DummyNode extends Node {}

  class NumeralNode( valueIn: Int ) extends Node {

    var value = valueIn

  }

  class BinaryOperatorNode( opType: Operations.Value ) extends Node {

    var value = opType
    var left: Option[Node] = None
    var right: Option[Node] = None

    def setLeft( node: Node ) = { this.left = Some( node ) }
    def setRight( node: Node ) = { this.right = Some( node ) }

  }

  class UnaryOperatorNode( opType: Operations.Value ) extends Node {

    var value = opType
    var operand: Option[Node] = None

    def setOperand( node: Node ) = { this.operand = Some( node ) }

  }

}
