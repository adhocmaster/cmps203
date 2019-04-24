# While in Haskell

# Added feature: RemExp (remainder) and IncExp (Increment operator)

# Test File: test.hs

# Comprehensive Test Cases: (All from test.hs plus some more to test in console)

## Test Arith with variables.
```haskell
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalArith( VarExp "x", Map.empty )
evalArith( VarExp "x", s )
evalArith( SumExp (VarExp "x") (IntExp 100), s )
evalArith( SubExp (VarExp "x") (VarExp "x"), s )
evalArith( MulExp (VarExp "x") (IntExp 100), s )
evalArith( RemExp (VarExp "x") (IntExp 2), s )
evalArith( RemExp (VarExp "x") (IntExp 0), s )
```

## Test Booleans
```haskell
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalBool( BoolExp True, s )
evalBool( EQExp (IntExp 2) (VarExp "x"), s)
evalBool( EQExp (IntExp 3) (VarExp "x"), s)
evalBool( LtExp (IntExp 3) (VarExp "x"), s)
evalBool( LtExp (IntExp 1) (VarExp "x"), s)

evalBool( OrExp ( EQExp (IntExp 3) (VarExp "x") ) (EQExp (IntExp 2) (VarExp "x") ), s)
evalBool( AndExp ( EQExp (IntExp 3) (VarExp "x") ) (EQExp (IntExp 2) (VarExp "x") ), s)
```

## Assignment test 
```haskell
let s = evalStmt  ((Assign "x" (IntExp 2)), Map.empty)
evalStmt( Seq (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s)
evalStmt( Skip True, s)
-- Decrement
evalStmt( (Assign "x" (SubExp (VarExp "x") (IntExp 1) ) ), s)
-- Square
evalStmt( (Assign "y" (MulExp (VarExp "x") (VarExp "x") )), s)
```
## If test
```haskell
let s = evalStmt  ((Assign "x" (IntExp 2)), Map.empty)
evalStmt( If (BoolExp True) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
evalStmt( If (BoolExp False) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
```

## Mighty While test
```haskell
let s = evalStmt  ((Assign ("x") (IntExp 1)), Map.empty)
let s2 = evalStmt( While (LtExp (VarExp "x") (IntExp 11)) (Assign "x" ( SumExp (VarExp "x") (IntExp 1) )), s)
let s3 = evalStmt( While (LtExp (VarExp "x") (IntExp -10)) (Assign "x" ( SumExp (VarExp "x") (IntExp 1) )), s)
```

## Test for sequence of first 10 positive integers
```haskell
let sx = evalStmt  ((Assign ("x") (IntExp 1)), Map.empty)
let sSum = evalStmt  ((Assign ("sum") (IntExp 0)), sx)
let s4 = evalStmt( While (LtExp (VarExp "x") (IntExp 11)) (Seq (Assign "sum" (SumExp (VarExp "x") (VarExp "sum"))) (Assign "x" ( SumExp (VarExp "x") (IntExp 1) )) ), sSum)
```
## Test Increment Expression
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalStmt(Inc "x",s)
