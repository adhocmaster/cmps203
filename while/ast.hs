module AST where 
        
import Data.Map (Map, (!))
import qualified Data.Map as Map

import qualified Data.Void as Void

import Data.Maybe as Maybe

type S = Map.Map String Int
type Var = String

data E = IntExp Int
        | VarExp Var
        | SumExp E E
        | SubExp E E
        | MulExp E E

data B = BoolExp Bool
        | EQExp E E
        | LtExp E E
        | NotExp B
        | AndExp B B
        | OrExp B B


data C = Skip Bool
        | Assign Var E
        | Seq C C
        | If B C C
        | While B C
