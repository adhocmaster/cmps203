data Exp = IntExp Int
            | SumExp Exp Exp
            | MulExp Exp Exp

eval :: Exp -> Int
eval (IntExp n) = n
eval (SumExp e1 e2) = (eval e1) + (eval e2)
eval (MulExp e1 e2) = (eval e1) * (eval e2)

test_eval :: Bool
test_eval = let e = MulExp (SumExp (IntExp 3) (IntExp 5)) (IntExp 2)
            in (eval e) == 16 -- True