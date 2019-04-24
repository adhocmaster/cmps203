import AST

import Data.Void
import qualified Data.Map as Map

s = Map.empty

evalArith :: E -> Int
evalArith (IntExp n) = n

evalBool :: B -> Bool
evalBool (BoolExp any) = any 

evalStmt :: (C, S) -> S
evalStmt (Skip _, s) = s
evalStmt (Assign name e, s) = Map.insert name ( evalArith(e) ) s
evalStmt (Seq c1 c2, s) =
    let s1 = evalStmt(c1, s)
    in evalStmt(c2, s1)



