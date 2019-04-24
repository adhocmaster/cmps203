import Interpreter
import AST
import qualified Data.Map as Map

testAssignAndArith :: Bool
testAssignAndArith = 
    let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
        sum = evalArith( SumExp (VarExp "x") (IntExp 100), s )
        sub = evalArith( SubExp (VarExp "x") (VarExp "x"), s )
        mul = evalArith( MulExp (VarExp "x") (IntExp 100), s )
    in ((sum == 102) && (sub == 0) && (mul == 200 )) == True

testBoolean :: Bool
testBoolean = 
    let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
        t1 = evalBool( BoolExp True, s )
        t2 = evalBool( EQExp (IntExp 2) (VarExp "x"), s)
        f1 = evalBool( EQExp (IntExp 3) (VarExp "x"), s)
        f2 = evalBool( LtExp (IntExp 3) (VarExp "x"), s)
        t3 = evalBool( LtExp (IntExp 1) (VarExp "x"), s)

        t4 = evalBool( OrExp ( EQExp (IntExp 3) (VarExp "x") ) (EQExp (IntExp 2) (VarExp "x") ), s)
        f3 = evalBool( AndExp ( EQExp (IntExp 3) (VarExp "x") ) (EQExp (IntExp 2) (VarExp "x") ), s)
    in ((t1 == True) && (t2 == True) && (t3 == True) && (t4 == True) && (f1 == False) && (f2 == False) && (f3 == False)) == True

testAssignment :: Bool
testAssignment = 
    let s = evalStmt  ((Assign "x" (IntExp 2)), Map.empty)
        s1 = evalStmt( Seq (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s)
        s2 = evalStmt( Skip True, s)
        -- Decrement
        s3 = evalStmt( (Assign "x" (SubExp (VarExp "x") (IntExp 1) ) ), s)
        -- Square
        s4 = evalStmt( (Assign "y" (MulExp (VarExp "x") (VarExp "x") )), s)
    in ( (s1 Map.! "y" == 3) && (Map.isSubmapOf s s2 && Map.isSubmapOf s2 s ) && (s3 Map.! "x" == 1) && (s4 Map.! "y" == 4) ) == True

testIf :: Bool
testIf = 
    let s = evalStmt  ((Assign "x" (IntExp 2)), Map.empty)
        s2 = evalStmt( If (BoolExp True) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
        s3 = evalStmt( If (BoolExp False) (Assign "y" (IntExp 3)) (Assign "z" (IntExp 10)), s )
        s4 = evalStmt( If (EQExp (VarExp "x") (IntExp 2)) (Assign "y" (IntExp 30)) (Assign "z" (IntExp 10)), s )
    in ( (s2 Map.! "y" == 3) && (s3 Map.! "z" == 10) && (s4 Map.! "y" == 30) ) == True

testWhile :: Bool
testWhile = 
    -- sum of first 10 positive integers
    let sx = evalStmt  ((Assign ("x") (IntExp 1)), Map.empty)
        sSum = evalStmt  ((Assign ("sum") (IntExp 0)), sx)
        sSumTrue = evalStmt( While (LtExp (VarExp "x") (IntExp 11)) (Seq (Assign "sum" (SumExp (VarExp "x") (VarExp "sum"))) (Assign "x" ( SumExp (VarExp "x") (IntExp 1) )) ), sSum)
        sSumFalse = evalStmt( While (LtExp (VarExp "x") (IntExp 0)) (Seq (Assign "sum" (SumExp (VarExp "x") (VarExp "sum"))) (Assign "x" ( SumExp (VarExp "x") (IntExp 1) )) ), sSum)
    in ( (sSumTrue Map.! "sum" == 55 ) && (sSumFalse Map.! "sum" == 0 )  ) == True

testRemainder :: Bool
testRemainder = 
    let s = evalStmt  ((Assign "x" (IntExp 2)), Map.empty)
        rem = evalArith( RemExp (VarExp "x") (IntExp 2), s )
    in (rem == 0) == True

testInc :: Bool
testInc = 
    let s = evalStmt  ((Assign ("x") (IntExp 2)), Map.empty)
        s2 = evalStmt(Inc "x",s)
    in (s2 Map.! "x" == 3) == True

testAll :: Bool
testAll = testAssignAndArith && testAssignment && testBoolean && testIf && testInc && testRemainder && testWhile