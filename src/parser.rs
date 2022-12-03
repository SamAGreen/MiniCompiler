// E => T E'
// E' => + T E'
// T => F T'
// T' => * F T'
// F => N | (E)

use crate::tokenizer::{Tokenizer,TokenT};
use crate::ast::{Exp};

pub struct Parser {
    t: Tokenizer
}

impl Parser {
    pub fn parse(&mut self) -> Option<Box<Exp>> {
        self.parse_e()
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
    fn test_1() {
        let mut p = Parser { t: tokenizer("(1)*0") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*0"));
    }

    #[test]
    fn test_2() {
        let mut p = Parser { t: tokenizer("1 + 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+0"));
    }

    #[test]
    fn test_3() {
        let mut p = Parser { t: tokenizer("1 + (0) ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+0"));
    }

    #[test]
    fn test_4() {
        let mut p = Parser { t: tokenizer("1 + 2 * 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1+2*0"));
    }

    #[test]
    fn test_5() {
        let mut p = Parser { t: tokenizer("1 * 2 + 0 ") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*2+0"));
    }

    #[test]
    fn test_6() {
        let mut p = Parser { t: tokenizer("(1* ( 1 + 2) * 0 )") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("1*(1+2)*0"));
    }

    #[test]
    fn test_7() {
        let mut p = Parser { t: tokenizer("(1 + 2) * 0 + 2") };
        let ret = p.parse();
        assert!(ret.unwrap().pretty().eq("(1+2)*0+2"));
    }
}
