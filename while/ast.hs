module AST where 
import Data.Map as Map
import Data.Void

type S = Map.Map String Int
type Var = String

data E = IntExp Int
        | X Var
        | SumExp E E
        | SubExp E E
        | MulExp E E

data B = BoolExp Bool
        | EQExp E E
        | LtExp E E
        | NotExp B
        | AndExp B B
        | OrExp B B


data C = Skip Void
        | Assign Var E
        | Seq C C
        | If B C C
        | While B C
