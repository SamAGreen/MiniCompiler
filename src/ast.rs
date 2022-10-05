pub enum ExpType {
    INT,
    PLUS,
    MULT
}

pub trait Exp {
    fn eval(&self) -> i32;
    fn pretty(&self) -> String;
    fn get_type(&self) -> ExpType;
}

pub struct IntExp {
    pub val: i32
}

impl Exp for IntExp {
    fn eval(&self) -> i32 {
        self.val
    }

    fn pretty(&self) -> String {
        self.val.to_string()
    }

    fn get_type(&self) -> ExpType {
        ExpType::INT
    }
}

pub struct PlusExp {
    pub e1: Box<dyn Exp>,
    pub e2: Box<dyn Exp>
}

impl Exp for PlusExp {
    fn eval(&self) -> i32 {
        self.e1.eval() + self.e2.eval()
    }

    fn pretty(&self) -> String {
        self.e1.pretty() + "+" + &*self.e2.pretty()
    }

    fn get_type(&self) -> ExpType {
        ExpType::PLUS
    }
}

pub struct MultExp {
    pub e1: Box<dyn Exp>,
    pub e2: Box<dyn Exp>
}

impl Exp for MultExp {
    fn eval(&self) -> i32 {
        self.e1.eval() * self.e2.eval()
    }

    // Just want to see if this works
    fn pretty(&self) -> String {
        let mut s = "".to_owned();
        match self.e1.get_type() {
            ExpType::PLUS => {
                s.push('(');
                s+= &*self.e1.pretty();
                s.push(')')
            },
            _ => s+= &*self.e2.pretty()
        };
        s.push('*');
        match self.e2.get_type() {
            ExpType::PLUS => {
                s.push('(');
                s+= &*self.e2.pretty();
                s.push(')')
            },
            _ => s+= &*self.e2.pretty()
        };

        s
    }

    fn get_type(&self) -> ExpType {
        ExpType::MULT
    }
}

pub fn test_pretty() {
    let zero = Box::new( IntExp { val: 0 });
    let one = Box::new(IntExp { val: 1});
    let two = Box::new( IntExp { val: 2 });
    let t0 = Box::new(PlusExp {e1: one, e2: two});
    let t1 = MultExp { e1: t0, e2: zero };
    println!("{}", t1.pretty())
}