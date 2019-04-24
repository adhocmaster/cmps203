# While in Haskell

# Test Cases:

## Assignment test 
```haskell
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalStmt( Seq (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s)
evalStmt( Skip True, s)
```
## If test
```haskell
evalStmt( If (BoolExp True) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
evalStmt( If (BoolExp False) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
```

## Test Arith with variables.
```haskell
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalArith( VarExp "x", Map.empty )
evalArith( VarExp "x", s )
evalArith( SumExp (VarExp "x") (IntExp 100), s )
evalArith( SubExp (VarExp "x") (IntExp 100), s )
evalArith( MulExp (VarExp "x") (IntExp 100), s )
```
