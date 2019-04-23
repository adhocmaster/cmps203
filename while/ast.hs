module AST where 
data L = X Int
data E = IntExp Int
        | Var L
        | SumExp E E
        | SubExp E E
        | MulExp E E

data B = True | False
        | EQExp E E
        | OrdExp E E
        | NotExp B
        | AndExp B B
        | OrExp B B
