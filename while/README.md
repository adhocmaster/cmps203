# While in Haskell

# Test Cases:

1. 
let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
evalStmt( Seq (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s)
evalStmt( Skip True, s)
2. 
