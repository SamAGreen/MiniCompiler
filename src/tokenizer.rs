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

pub fn show_tok(t: &TokenT) -> &'static str {
    return match t {
        TokenT::EOS => "EOS",
        TokenT::ZERO => "ZERO",
        TokenT::ONE => "ONE",
        TokenT::TWO => "TWO",
        TokenT::OPEN => "OPEN",
        TokenT::CLOSE => "CLOSE",
        TokenT::PLUS => "PLUS",
        TokenT::MULT => "MULT"
    };
}

struct Tokenize {
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

    pub fn scan(&mut self) -> Vec<TokenT> {
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

    pub fn show(&mut self) -> String {
        let mut s: String = String::from("");

        let test = self.scan();
        for v in test.iter() {
            s.push_str(show_tok(v));
            s.push(';');
        }
        s
    }
}
// Not super sure about this, it's inheritance through composition, good enough for now
pub struct Tokenizer {
    t : Tokenize,
    token : TokenT,
}

impl Tokenizer {
    pub fn next_token(&mut self) {
        self.token = self.t.next()
    }
}

/** TEST STUFF **/

// Just straight up stole this from stackoverflow
fn do_vectors_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn test_tokenize() {
    // Prep
    let mut t0_tokenize = Tokenize { s: String::from("1+1"), pos: 0 };
    let t0_vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
    let t0_string = String::from("ONE;PLUS;ONE;EOS;");

    let mut t1_tokenize = Tokenize { s: String::from("1 + 1"), pos: 0 };
    let t1_vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE, TokenT::EOS];
    let t1_string = String::from("ONE;PLUS;ONE;EOS;");

    let mut t2_tokenize = Tokenize { s: String::from("1 * 1"), pos: 0 };
    let t2_vector = vec![TokenT::ONE, TokenT::MULT, TokenT::ONE, TokenT::EOS];
    let t2_string = String::from("ONE;MULT;ONE;EOS;");

    let mut t3_tokenize = Tokenize { s: String::from("1 * 2 + 4"), pos: 0 };
    let t3_vector = vec![TokenT::ONE, TokenT::MULT, TokenT::TWO, TokenT::PLUS, TokenT::EOS];
    let t3_string = String::from("ONE;MULT;TWO;PLUS;EOS;");

    let mut t4_tokenize = Tokenize { s: String::from("((1+2)*2)+1"), pos: 0 };
    let t4_vector = vec![
        TokenT::OPEN, TokenT::OPEN, TokenT::ONE, TokenT::PLUS, TokenT::TWO, TokenT::CLOSE,
        TokenT::MULT, TokenT::TWO, TokenT::CLOSE, TokenT::PLUS, TokenT::ONE, TokenT::EOS
    ];
    let t4_string = String::from("OPEN;OPEN;ONE;PLUS;TWO;CLOSE;MULT;TWO;CLOSE;PLUS;ONE;EOS;");


    // Test vector matching
    println!("\n**** Vector Matching Tests ****\n");
    println!("Matching t0 to t0. Expected: true, result: {}", do_vectors_match(&t0_vector, &t0_vector));
    println!("Matching t0 to t1. Expected: true, result: {}", do_vectors_match(&t0_vector, &t1_vector));
    println!("Matching t0 to t2. Expected: false, result: {}", do_vectors_match(&t0_vector, &t2_vector));
    println!("Matching t0 to t3. Expected: false, result: {}", do_vectors_match(&t0_vector, &t3_vector));
    println!("Matching t0 to t4. Expected: false, result: {}", do_vectors_match(&t0_vector, &t4_vector));

    // Test Tokenization
    println!("\n****** Tokenization Tests ******\n");
    println!("Scan:");
    let t0_scan_comp_result = do_vectors_match(&(t0_tokenize.scan()), &t0_vector);
    let t1_scan_comp_result = do_vectors_match(&(t1_tokenize.scan()), &t1_vector);
    let t2_scan_comp_result = do_vectors_match(&(t2_tokenize.scan()), &t2_vector);
    let t3_scan_comp_result = do_vectors_match(&(t3_tokenize.scan()), &t3_vector);
    let t4_scan_comp_result = do_vectors_match(&(t0_tokenize.scan()), &t0_vector);

    t0_tokenize.pos=0;
    let t0_scan_with_t2_comp_result = do_vectors_match(&(t0_tokenize.scan()), &t2_vector);

    println!("Matching t0 scan with vector. Expected: true, result: {}", t0_scan_comp_result);
    println!("Matching t1 scan with vector. Expected: true, result: {}", t1_scan_comp_result);
    println!("Matching t2 scan with vector. Expected: true, result: {}", t2_scan_comp_result);
    println!("Matching t3 scan with vector. Expected: true, result: {}", t3_scan_comp_result);
    println!("Matching t4 scan with vector. Expected: true, result: {}", t4_scan_comp_result);
    println!("Matching t0 scan with t2_vector. Expected: false, result: {}", t0_scan_with_t2_comp_result);

    // Reset tokenize
    t0_tokenize.pos=0;
    t1_tokenize.pos=0;
    t2_tokenize.pos=0;
    t3_tokenize.pos=0;
    t4_tokenize.pos=0;


    println!("\nShow:");
    let t0_show_result= t0_tokenize.show();
    let t1_show_result = t1_tokenize.show();
    let t2_show_result = t2_tokenize.show();
    let t3_show_result = t3_tokenize.show();
    let t4_show_result = t4_tokenize.show();

    println!("Matching t0 show with string.     Expected: true: result: {}", t0_show_result.eq(&t0_string));
    println!("Matching t0 show with t1_string.  Expected: true: result: {}",  t0_show_result.eq(&t1_string));
    println!("Matching t1 show with string.     Expected: true: result: {}", t1_show_result.eq(&t1_string));
    println!("Matching t2 show with string.     Expected: true: result: {}", t2_show_result.eq(&t2_string));
    println!("Matching t3 show with string.     Expected: true: result: {}", t3_show_result.eq(&t3_string));
    println!("Matching t4 show with string.     Expected: true: result: {}", t4_show_result.eq(&t4_string));
    println!("Matching t0 show with t2_string.  Expected: false: result: {}", t0_show_result.eq(&t2_string));
}