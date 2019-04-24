module Interpreter where 
import AST

import qualified Data.Map as Map
import Debug.Trace


evalArith :: (E, S) -> Int
evalArith (IntExp n, s) = n
evalArith (VarExp name, s)
    | ( Map.lookup name s ) == Nothing = error ( "Undefined variable: " ++ name )
    | otherwise = s Map.! name
evalArith (SumExp e1 e2, s) = (evalArith (e1, s)) + (evalArith (e2, s))
evalArith (SubExp e1 e2, s) = (evalArith (e1, s)) - (evalArith (e2, s))
evalArith (MulExp e1 e2, s) = (evalArith (e1, s)) * (evalArith (e2, s))
evalArith (RemExp e1 e2, s)
    | evalArith (e2, s) == 0 = error ( "Division by 0" )
    | otherwise = (evalArith (e1, s)) `rem` (evalArith (e2, s))

evalBool :: (B, S) -> Bool
evalBool (BoolExp any, s) = any 
evalBool (EQExp e1 e2, s) = (evalArith (e1, s)) == (evalArith (e2, s))
evalBool (LtExp e1 e2, s) = (evalArith (e1, s)) < (evalArith (e2, s))
evalBool (NotExp b, s) = not ( evalBool(b, s) )
evalBool (AndExp b1 b2, s) = (evalBool (b1, s)) && (evalBool (b2, s))
evalBool (OrExp b1 b2, s) = (evalBool (b1, s)) || (evalBool (b2, s))

evalStmt :: (C, S) -> S
evalStmt (Skip _, s) = s
evalStmt (Assign name e, s) = Map.insert name ( evalArith(e, s) ) s
evalStmt (Seq c1 c2, s) =
    let s1 = evalStmt(c1, s)
    in evalStmt(c2, s1)

evalStmt (If cond c1 c2, s)
    | (evalBool(cond, s)) == True = evalStmt(c1, s)
    | otherwise = evalStmt(c2, s)

evalStmt (While cond c, s)
    | (evalBool(cond,s )) == True = trace("trace before loop s: " ++ show s) $ evalStmt(While cond c, (evalStmt(c, s)))
    | otherwise = s

evalStmt (Inc name, s)
    | ( Map.lookup name s ) == Nothing = error ( "Undefined variable: " ++ name )
    | otherwise = Map.insert name (s Map.! name + 1) s


