import AST

import Data.Void
import qualified Data.Map as Map

s = Map.empty

evalArith :: (E, S) -> Int
evalArith (IntExp n, s) = n
evalArith (VarExp name, s)
    | ( Map.lookup name s ) == Nothing = error ( "Undefined variable: " ++ name )
    | otherwise = s Map.! name
evalArith (SumExp e1 e2, s) = (evalArith (e1, s)) + (evalArith (e2, s))
evalArith (SubExp e1 e2, s) = (evalArith (e1, s)) - (evalArith (e2, s))
evalArith (MulExp e1 e2, s) = (evalArith (e1, s)) * (evalArith (e2, s))

evalBool :: B -> Bool
evalBool (BoolExp any) = any 

evalStmt :: (C, S) -> S
evalStmt (Skip _, s) = s
evalStmt (Assign name e, s) = Map.insert name ( evalArith(e, s) ) s
evalStmt (Seq c1 c2, s) =
    let s1 = evalStmt(c1, s)
    in evalStmt(c2, s1)

evalStmt (If cond c1 c2, s)
    | (evalBool(cond)) == True = evalStmt(c1, s)
    | otherwise = evalStmt(c2, s)

evalStmt (While cond c, s)
    | (evalBool(cond)) == True = evalStmt(While cond c, (evalStmt(c, s)))
    | otherwise = s


