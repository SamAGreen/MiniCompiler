// E => T E'
// E' => + T E'
// T => F T'
// T' => * F T'
// F => N | (E)

use crate::tokenizer::{Tokenizer,TokenT};
use crate::ast::{Exp,IntExp,PlusExp,MultExp};
use crate::ast::ExpType::{MULT, PLUS};


struct Parser {
    t: Tokenizer
}

impl Parser {
    pub fn parse(&mut self) -> Option<Box<dyn Exp>> {
        self.parse_e()
    }

    fn parse_e(&mut self) -> Option<Box<dyn Exp>> {
        let t = self.parse_t();
        return match t {
            Some(left) => self.parse_e2(left),
            None =>  t
        }
    }

    fn parse_e2(&mut self, left: Box<dyn Exp>) -> Option<Box<dyn Exp>> {
        if self.t.token == TokenT::PLUS {
            self.t.next_token();

            let t = self.parse_t();
            return match t {
                Some(right) => self.parse_e2(Box::new(PlusExp {e1:left, e2: right})),
                None => t
            }
        }

        Some(left)
    }

    fn parse_t(&mut self) -> Option<Box<dyn Exp>> {
        let f = self.parse_f();

        return match f {
            Some(exp) => self.parse_t2(exp),
            None => f
        }
    }

    fn parse_t2(&mut self, left: Box<dyn Exp>) -> Option<Box<dyn Exp>> {
        if self.t.token == TokenT::MULT {
            self.t.next_token();

            let t = self.parse_f();
            return match t {
                Some(right) => self.parse_t2(Box::new(MultExp {e1: left, e2: right})),
                None => t
            }
        }
        Some(left)
    }

    fn parse_f(&mut self) -> Option<Box<dyn Exp>> {
        todo!()
    }
}