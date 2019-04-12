###  Source package: com.adhocmaster.cmps203updated.arith2

# How to run:

1. Import project to a eclipse IDE having the Scala extension installed
2. Run App.Scala as a "Scala Application"

# Here goes the test report:

Arith AST and Interpreter in Scala by Golam Md Muktadir

Test: TestNumeral
Result: 5

Test: TestBinaryNoOperand with expression: +
Error: Incomplete AST

Test: TestBinaryLeftOperand with expression: 5 +
Error: Incomplete AST

Test: TestBinaryRightOperand with expression: * 7
Error: Incomplete AST

Test: TestAdd with expression: 5 + 7
Result: 12

Test: TestAdd with expression: 5 * (-7)
Result: -35

Test: TestExpressionFromSlides with expression: (3 + 5) * 2 
Result: 16

Test: TestExpressionFromSlidesAugmented with expression: ((3 + 5) * 2) + ((3 + 5) * 2) 
Result: 32

Test: TestMod with expression: 5 % 2 
Result: 1

Test: TestModByZero with expression: 5 % 0 
Error: / by zero
