// E => T E'
// E' => + T E'
// T => F T'
// T' => * F T'
// F => N | (E)

use crate::tokenizer::{Tokenizer,TokenT};
use crate::ast::{Exp,IntExp,PlusExp,MultExp};


struct Parser {
    t: Tokenizer
}

impl Parser {
    fn parse_e<T:Exp>(&self) -> Option<T> {
    todo!()
    }

    fn parse_e2<T:Exp>(&self, left: T) -> Option<T> {
        todo!()
    }

    fn parse_t<T:Exp>(&self) -> Option<T> {
        todo!()
    }

    fn parse_t2<T:Exp>(&self, left: T) -> Option<T> {
        todo!()
    }

    fn parse_f<T:Exp>(&self) -> Option<T> {
        todo!()
    }
}