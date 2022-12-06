use crate::tokenizer::{Tokenizer,TokenT};
use crate::ast::{Exp};

pub struct Parser {
    t: Tokenizer
}

impl Parser {
    pub fn parse(&mut self) -> Option<Box<Exp>> {
        let mut parsed = self.parse_e();
        if self.t.token != TokenT::EOS {
            parsed = None
        }
        match parsed{
            Some(p) => {
                println!("Parsing was successful!");
                Some(p)
            },
            None => {
                println!("Parsing was unsuccessful!");
                None
            }
        }
    }

    fn parse_e(&mut self) -> Option<Box<Exp>> {
        let t = self.parse_t();
        if let Some(left) = t {
            self.parse_e2(left)
        } else {
            t
        }
    }

    fn parse_e2(&mut self, left: Box<Exp>) -> Option<Box<Exp>> {
        if self.t.token == TokenT::PLUS {
            self.t.next_token();

            let t = self.parse_t();
            return if let Some(right) = t {
                self.parse_e2(Box::new(Exp::Plus {e1:left, e2: right}))
            } else {
                t
            }
        }

        Some(left)
    }

    fn parse_t(&mut self) -> Option<Box<Exp>> {
        let f = self.parse_f();
        return if let Some(exp) = f {
            self.parse_t2(exp)
        } else {
            f
        }
    }

    fn parse_t2(&mut self, left: Box<Exp>) -> Option<Box<Exp>> {
        if self.t.token == TokenT::MULT {
            self.t.next_token();

            let t = self.parse_f();
            return if let Some(right) = t {
                self.parse_t2(Box::new(Exp::Mult { e1: left, e2: right }))
            } else {
                t
            }
        }
        Some(left)
    }

    fn parse_f(&mut self) -> Option<Box<Exp>> {
        return match self.t.token {
            TokenT::ZERO => {
                self.t.next_token();
                Some(Box::new(Exp::Int {val: 0}))
            },
            TokenT::ONE => {
                self.t.next_token();
                Some(Box::new(Exp::Int {val: 1}))
            },
            TokenT::TWO => {
                self.t.next_token();
                Some(Box::new(Exp::Int {val: 2}))
            },
            TokenT::OPEN => {
                self.t.next_token();
                let e = self.parse_e();

                if e.is_none() {
                    return e;
                }

                if self.t.token != TokenT::CLOSE {
                    return None;
                }

                self.t.next_token();
                e
            },
            _ => None

        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tokenizer::{tokenizer};
    use super::*;

    #[test]
    fn test_parse_success_1() {
        let mut p = Parser { t: tokenizer("(1)*0") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*0"));
    }

    #[test]
    fn test_parse_success_2() {
        let mut p = Parser { t: tokenizer("1 + 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+0"));
    }

    #[test]
    fn test_parse_success_3() {
        let mut p = Parser { t: tokenizer("1 + (0) ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+0"));
    }

    #[test]
    fn test_parse_success_4() {
        let mut p = Parser { t: tokenizer("1 + 2 * 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+2*0"));
    }

    #[test]
    fn test_parse_success_5() {
        let mut p = Parser { t: tokenizer("1 * 2 + 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*2+0"));
    }

    #[test]
    fn test_parse_success_6() {
        let mut p = Parser { t: tokenizer("(1* ( 1 + 2) * 0 )") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*(1+2)*0"));
    }

    #[test]
    fn test_parse_success_7() {
        let mut p = Parser { t: tokenizer("(1 + 2) * 0 + 2") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("(1+2)*0+2"));
    }

    #[test]
    fn test_parse_failure_1() {
        let mut p = Parser { t: tokenizer("(1*0") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_2() {
        let mut p = Parser { t: tokenizer("1+ + 10 ") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_3() {
        let mut p = Parser { t: tokenizer("1 + (0 ") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_4() {
        let mut p = Parser { t: tokenizer("(1 + 2 * 0 ") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_5() {
        let mut p = Parser { t: tokenizer("1 * 2 + 0)") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_6() {
        let mut p = Parser { t: tokenizer("(11*2+ ( 1 + 2 * 0 )") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_7() {
        let mut p = Parser { t: tokenizer("((((1 + 2))) * 0 + 2") };
        let ret = p.parse();
        assert!(ret.is_none());
    }

    #[test]
    fn test_parse_failure_8() {
        let mut p = Parser { t: tokenizer("11") };
        let ret = p.parse();
        assert!(ret.is_none());
    }
}
