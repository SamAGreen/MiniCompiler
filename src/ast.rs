#[derive(Clone, PartialEq)]
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
    };
}

pub fn test_pretty() {
    let zero = Box::new(Exp::Int { val: 0 });
    let one = Box::new(Exp::Int { val: 1 });
    let two = Box::new(Exp::Int { val: 2 });
    let t0 = Box::new(Exp::Plus { e1: one, e2: two });
    let t1 = Exp::Mult { e1: t0, e2: zero };
    println!("{}", t1.pretty())
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn get_some_expressions() -> HashMap<String, Box<Exp>> {
        let zero = Box::new(Exp::Int { val: 0 });
        let one = Box::new(Exp::Int { val: 1 });
        let two = Box::new(Exp::Int { val: 2 });

        let opo = Box::new(Exp::Plus { e1: one.clone(), e2: one.clone() });
        let opo_times_two = Box::new(Exp::Mult { e1: opo.clone(), e2: two.clone() });
        let zero_plus_opo_tt = Box::new(Exp::Plus { e1: zero.clone(), e2: opo_times_two.clone() });

        let zto = Box::new(Exp::Mult { e1: zero.clone(), e2: one.clone() });
        let zto_plus_one = Box::new(Exp::Plus { e1: zto.clone(), e2: one.clone() });

        let opo_times_two_times_zto_plus_one = Box::new(Exp::Mult { e1: opo_times_two.clone(), e2: zto_plus_one.clone() });

        let expressions = HashMap::from([
            ("opo".to_string(), opo),
            ("opo_times_two".to_string(), opo_times_two),
            ("zero_plus_opo_tt".to_string(), zero_plus_opo_tt),
            ("zto".to_string(), zto),
            ("zto_plus_one".to_string(), zto_plus_one),
            ("opo_times_two_times_zto_plus_one".to_string(), opo_times_two_times_zto_plus_one)
        ]);
        expressions
    }

    #[test]
    fn test_eval_1() {
        let exp = get_some_expressions().remove("opo").unwrap();
        assert_eq!(2,exp.eval());
    }

    #[test]
    fn test_eval_2() {
        let exp = get_some_expressions().remove("opo_times_two").unwrap();
        assert_eq!(4,exp.eval());
    }

    #[test]
    fn test_eval_3() {
        let exp = get_some_expressions().remove("zero_plus_opo_tt").unwrap();
        assert_eq!(4,exp.eval());
    }

    #[test]
    fn test_eval_4() {
        let exp = get_some_expressions().remove("zto").unwrap();
        assert_eq!(0,exp.eval());
    }

    #[test]
    fn test_eval_5() {
        let exp = get_some_expressions().remove("zto_plus_one").unwrap();
        assert_eq!(1,exp.eval());
    }

    #[test]
    fn test_eval_6() {
        let exp = get_some_expressions().remove("opo_times_two_times_zto_plus_one").unwrap();
        assert_eq!(4,exp.eval());
    }

    #[test]
    fn test_pretty_1() {
        let exp = get_some_expressions().remove("opo").unwrap();
        assert_eq!("1+1",exp.pretty());
    }

    #[test]
    fn test_pretty_2() {
        let exp = get_some_expressions().remove("opo_times_two").unwrap();
        assert_eq!("(1+1)*2",exp.pretty());
    }

    #[test]
    fn test_pretty_3() {
        let exp = get_some_expressions().remove("zero_plus_opo_tt").unwrap();
        assert_eq!("0+(1+1)*2",exp.pretty());
    }

    #[test]
    fn test_pretty_4() {
        let exp = get_some_expressions().remove("zto").unwrap();
        assert_eq!("0*1",exp.pretty());
    }

    #[test]
    fn test_pretty_5() {
        let exp = get_some_expressions().remove("zto_plus_one").unwrap();
        assert_eq!("0*1+1",exp.pretty());
    }

    #[test]
    fn test_pretty_6() {
        let exp = get_some_expressions().remove("opo_times_two_times_zto_plus_one").unwrap();
        assert_eq!("(1+1)*2*(0*1+1)",exp.pretty());
    }
}