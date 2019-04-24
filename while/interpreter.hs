import AST

import Data.Void
import qualified Data.Map as Map

s = Map.empty

evalArith :: E -> Int
evalArith (IntExp n) = n

evalBool :: B -> Bool
evalBool (BoolExp any) = any 

evalStmt :: C -> IO ()
evalStmt (Skip a) = putStrLn "Skipped"
evalStmt (Assign name value) = do
    putStrLn "Assign called"