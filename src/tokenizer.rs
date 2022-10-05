#[derive(Debug, PartialEq, Eq)]
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

pub fn show_tok(t: &TokenT) -> String {
    return match t {
        TokenT::EOS => String::from("EOS"),
        TokenT::ZERO => String::from("ZERO"),
        TokenT::ONE => String::from("ONE"),
        TokenT::TWO => String::from("TWO"),
        TokenT::OPEN => String::from("OPEN"),
        TokenT::CLOSE => String::from("CLOSE"),
        TokenT::PLUS => String::from("PLUS"),
        TokenT::MULT => String::from("MULT")
    };
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
        let mut v: Vec<TokenT> = vec![];
        let mut t: TokenT;

        loop {
            t = self.next();
            v.push(t);

            if *v.last().unwrap() == TokenT::EOS {
                break
            }
        }

        v
    }

    fn show(&mut self) -> String {
        let mut s: String = String::from("");

        let test = self.scan();
        for v in test.iter() {
            s.push_str(&*show_tok(v));
            s.push(';');
        }
        s
    }
}
// Not super sure about this, it's inheritance through composition, good enough for now
pub struct Tokenizer {
    pub t : Tokenize,
    pub token : TokenT,
}

impl Tokenizer {
    pub fn next_token(&mut self) {
        self.token = self.t.next()
    }
}

pub fn tokenizer(s: String) -> Tokenizer {
    let mut t = Tokenizer { t: (Tokenize { s, pos: 0 }), token: TokenT::EOS };
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
        let mut tokenizer = Tokenizer{ t: Tokenize { s: String::from("1 + 1"), pos: 0 }, token: TokenT::EOS };
        let vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
        assert_eq!(tokenizer.t.scan(), vector);
    }

    #[test]
    fn test_tokenizer_next() {
        let mut tokenizer = Tokenizer{ t: Tokenize { s: String::from("1 + 1"), pos: 0 }, token: TokenT::EOS };

    }
}