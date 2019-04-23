import AST
evalArith :: E -> Int
evalArith (IntExp n) = n

evalBool :: B -> Bool
evalBool (BoolExp any) = any 