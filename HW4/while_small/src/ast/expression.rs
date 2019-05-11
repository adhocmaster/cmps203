#[allow(dead_code)]
pub mod expression {
    use E::*;
    
    #[derive(Debug)]
    pub enum E {

        IntE(i32),
        SumE(Box<E>, Box<E>),
        SubE(Box<E>, Box<E>),
        MulE(Box<E>, Box<E>)

    }

    pub fn evalE(e: E) -> i32 {

        let val = match e {

            IntE(n) => n,
            SumE(e1, e2) => evalE(*e1) + evalE(*e2),
            SubE(e1, e2) => evalE(*e1) - evalE(*e2),
            MulE(e1, e2) => evalE(*e1) * evalE(*e2)

        };

        val

    }

    pub fn SumExp(e1: E, e2:E) -> E {

        SumE(Box::new(e1), Box::new(e2) )

    }
    pub fn IntExp(n:i32) -> E {

        IntE(n)

    }
    pub fn SubExp(e1: E, e2:E) -> E {

        SubE(Box::new(e1), Box::new(e2) )

    }
    pub fn MulExp(e1: E, e2:E) -> E {

        MulE(Box::new(e1), Box::new(e2) )

    }

}