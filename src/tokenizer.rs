use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum TokenT {
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
}

// Makes it possible to TokenT.to_string()
impl std::fmt::Display for TokenT {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            TokenT::EOS => String::from("EOS"),
            TokenT::ZERO => String::from("ZERO"),
            TokenT::ONE => String::from("ONE"),
            TokenT::TWO => String::from("TWO"),
            TokenT::OPEN => String::from("OPEN"),
            TokenT::CLOSE => String::from("CLOSE"),
            TokenT::PLUS => String::from("PLUS"),
            TokenT::MULT => String::from("MULT")
        };
        write!(f, "{}", t)
    }
}

pub struct  Tokenize {
    s: String,
    pos: usize,
}

impl Tokenize {
    pub fn next(&mut self) -> TokenT {
        loop {
            let c = self.s.chars().nth(self.pos);

            if c.is_none() {
                return TokenT::EOS;
            }

            self.pos += 1;

            match c.unwrap() {
                '0' => return TokenT::ZERO,
                '1' => return TokenT::ONE,
                '2' => return TokenT::TWO,
                '(' => return TokenT::OPEN,
                ')' => return TokenT::CLOSE,
                '+' => return TokenT::PLUS,
                '*' => return TokenT::MULT,
                _ => continue
            }
        }
    }

    fn scan(&mut self) -> Vec<TokenT> {
        let curr_pos = self.pos;
        self.pos = 0;

        let mut v: Vec<TokenT> = vec![];
        let mut t: TokenT;

        loop {
            t = self.next();
            v.push(t);

            if *v.last().unwrap() == TokenT::EOS {
                break
            }
        }
        self.pos = curr_pos;
        v
    }

    fn show(&mut self) -> String {
        let mut s: String = String::from("");

        let tokens = self.scan();
        for v in tokens.iter() {
            s.push_str(&v.to_string());
            s.push(';');
        }
        s
    }
}

pub struct Tokenizer {
    pub t : Tokenize,
    pub token : TokenT,
}

impl Tokenizer {
    pub fn next_token(&mut self) {
        self.token = self.t.next()
    }
}

pub fn tokenizer(s: &str) -> Tokenizer {
    let mut t = Tokenizer { t: (Tokenize { s: s.to_string(), pos: 0 }), token: TokenT::EOS };
    t.next_token();
    t
}

/** TEST STUFF **/
#[cfg(test)]
mod tests {
    use super::*;

    // Testing Tokenize scan
    #[test]
    fn test_tokenize_scan_1() {
        let mut tokenize = Tokenize { s: String::from("1+1"), pos: 0 };
        let vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
        assert_eq!(tokenize.scan(), vector);
    }

    #[test]
    fn test_tokenize_scan_2() {
        let mut tokenize = Tokenize { s: String::from("1 + 1"), pos: 0 };
        let vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
        assert_eq!(tokenize.scan(), vector);
    }

    #[test]
    fn test_tokenize_scan_3() {
        let mut tokenize = Tokenize { s: String::from("1 * 1"), pos: 0 };
        let vector = vec![TokenT::ONE, TokenT::MULT, TokenT::ONE, TokenT::EOS];
        assert_eq!(tokenize.scan(), vector);
    }

    #[test]
    fn test_tokenize_scan_4() {
        let mut tokenize = Tokenize { s: String::from("1 * 2 + 4"), pos: 0 };
        let vector = vec![TokenT::ONE, TokenT::MULT, TokenT::TWO, TokenT::PLUS, TokenT::EOS];
        assert_eq!(tokenize.scan(), vector);
    }

    #[test]
    fn test_tokenize_scan_5() {
        let mut tokenize = Tokenize { s: String::from("((1+2)*2)+1"), pos: 0 };
        let vector = vec![
            TokenT::OPEN, TokenT::OPEN, TokenT::ONE, TokenT::PLUS, TokenT::TWO, TokenT::CLOSE,
            TokenT::MULT, TokenT::TWO, TokenT::CLOSE, TokenT::PLUS, TokenT::ONE, TokenT::EOS
        ];
        assert_eq!(tokenize.scan(), vector);
    }

    #[test]
    fn test_tokenize_scan_6() {
        let mut tokenize = Tokenize { s: String::from("1+1"), pos: 0 };
        let vector = vec![TokenT::ONE, TokenT::MULT, TokenT::ONE, TokenT::EOS];
        assert_ne!(tokenize.scan(), vector);
    }

    // Testing Tokenize show
    #[test]
    fn test_tokenize_show_1() {
        let mut tokenize = Tokenize { s: String::from("1+1"), pos: 0 };
        let string = String::from("ONE;PLUS;ONE;EOS;");
        assert_eq!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenize_show_2() {
        let mut tokenize = Tokenize { s: String::from("1 + 1"), pos: 0 };
        let string = String::from("ONE;PLUS;ONE;EOS;");
        assert_eq!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenize_show_3() {
        let mut tokenize = Tokenize { s: String::from("1 * 1"), pos: 0 };
        let string = String::from("ONE;MULT;ONE;EOS;");
        assert_eq!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenize_show_4() {
        let mut tokenize = Tokenize { s: String::from("1 * 2 + 4"), pos: 0 };
        let string = String::from("ONE;MULT;TWO;PLUS;EOS;");
        assert_eq!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenize_show_5() {
        let mut tokenize = Tokenize { s: String::from("((1+2)*2)+1"), pos: 0 };
        let string = String::from("OPEN;OPEN;ONE;PLUS;TWO;CLOSE;MULT;TWO;CLOSE;PLUS;ONE;EOS;");
        assert_eq!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenize_show_6() {
        let mut tokenize = Tokenize { s: String::from("1 + 1"), pos: 0 };
        let string = String::from("ONE;MULT;ONE;EOS;");
        assert_ne!(tokenize.show(),string)
    }

    #[test]
    fn test_tokenizer_show() {
        let mut tokenizer = Tokenizer{ t: Tokenize { s: String::from("1 + 1"), pos: 0 }, token: TokenT::EOS };
        let string = String::from("ONE;PLUS;ONE;EOS;");
        assert_eq!(tokenizer.t.show(),string);
    }

    #[test]
    fn test_tokenizer_scan() {
        let mut tokenizer = tokenizer("1 + 1");
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::PLUS);

        let vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
        assert_eq!(tokenizer.t.scan(), vector);

        assert_eq!(tokenizer.token,TokenT::PLUS);
    }

    #[test]
    fn test_tokenizer_next_1() {
        let mut tokenizer = tokenizer("1 + 1");
        assert_eq!(tokenizer.token,TokenT::ONE);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::PLUS);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::ONE);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::EOS);
    }

    #[test]
    fn test_tokenizer_next_2() {
        let mut tokenizer = tokenizer("%&A1 * 2");
        assert_eq!(tokenizer.token,TokenT::ONE);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::MULT);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::TWO);
        tokenizer.next_token();
        assert_eq!(tokenizer.token,TokenT::EOS);
    }
}