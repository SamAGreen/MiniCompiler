pub enum Exp {
    Int {
        val: i32
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
    Mult {
        e1: Box<Exp>,
        e2: Box<Exp>,
    },
}

impl Exp {
    pub fn eval(&self) -> i32 {
        return match self {
            Exp::Int { val } => { *val }
            Exp::Plus { e1, e2 } => { e1.eval() + e2.eval() }
            Exp::Mult { e1, e2 } => { e1.eval() * e2.eval() }
        };
    }

    pub fn pretty(&self) -> String {
        return match self {
            Exp::Int { val } => { val.to_string() }
            Exp::Plus { e1, e2 } => { e1.pretty() + "+" + &*e2.pretty() }
            Exp::Mult { e1, e2 } => { pretty_mult(e1) + "*" + &*pretty_mult(e2) }
        };
    }
}

fn pretty_mult(e: &Box<Exp>) -> String {
    return match **e {
        Exp::Plus { .. } => {
            "(".to_string() + &*e.pretty() + ")"
        }
        _ => e.pretty()
    }
}

pub fn test_pretty() {
    let zero = Box::new(Exp::Int { val: 0 });
    let one = Box::new(Exp::Int { val: 1 });
    let two = Box::new(Exp::Int { val: 2 });
    let t0 = Box::new(Exp::Plus { e1: one, e2: two });
    let t1 = Exp::Mult { e1: t0, e2: zero };
    println!("{}", t1.pretty())
}