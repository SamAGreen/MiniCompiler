#[derive(Debug, PartialEq, Eq)]
pub enum TokenT {
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT
}

pub fn show_tok(t : TokenT) -> &'static str {
    return match t {
        TokenT::ZERO=> "ZERO",
        TokenT::ONE=> "ONE",
        TokenT::TWO=> "TWO",
        TokenT::OPEN=> "OPEN",
        TokenT::CLOSE=> "CLOSE",
        TokenT::PLUS=> "PLUS",
        TokenT::MULT=> "MULT"
    }
}

pub struct Tokenize {
    s: String
}

impl Tokenize {

    pub fn scan(&self) -> Vec<TokenT> {
        let mut v: Vec<TokenT> = vec![];

        for c in self.s.chars(){
            match c {
                '0' => v.push(TokenT::ZERO),
                '1' => v.push(TokenT::ONE),
                '2' => v.push(TokenT::TWO),
                '(' => v.push(TokenT::OPEN),
                ')' => v.push(TokenT::CLOSE),
                '+' => v.push(TokenT::PLUS),
                '*' => v.push(TokenT::MULT),
                _ => break
            }
        }

        v
    }
}

// Just straight up stole this from stackoverflow
fn do_vectors_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

pub fn test_some_things() {
    // Prep
    let t0_tokenize = Tokenize { s: String::from("1+1") };
    let t0_vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE];

    let t1_tokenize = Tokenize { s: String::from("1 + 1") };
    let t1_vector = vec![TokenT::ONE, TokenT::PLUS, TokenT::ONE];

    let t2_tokenize = Tokenize { s: String::from("1 * 1") };
    let t2_vector = vec![TokenT::ONE, TokenT::MULT, TokenT::ONE];

    let t3_tokenize = Tokenize { s: String::from("1 * 2 + 4") };
    let t3_vector = vec![TokenT::ONE, TokenT::MULT, TokenT::TWO, TokenT::PLUS];

    let t4_tokenize = Tokenize { s: String::from("((1+2)*2)+1") };
    let t4_vector = vec![
        TokenT::OPEN, TokenT::OPEN, TokenT::ONE, TokenT::PLUS, TokenT::TWO, TokenT::CLOSE,
        TokenT::MULT, TokenT::TWO, TokenT::CLOSE, TokenT::PLUS, TokenT::ONE
    ];

    // Test vector matching
    println!("**** Vector Matching Tests ****");
    println!("Matching t0 to t0. Expected: true, result: {}", do_vectors_match(&t0_vector, &t0_vector));
    println!("Matching t0 to t1. Expected: true, result: {}", do_vectors_match(&t0_vector, &t1_vector));
    println!("Matching t0 to t2. Expected: false, result: {}", do_vectors_match(&t0_vector, &t2_vector));
    println!("Matching t0 to t3. Expected: false, result: {}", do_vectors_match(&t0_vector, &t3_vector));
    println!("Matching t0 to t4. Expected: false, result: {}", do_vectors_match(&t0_vector, &t4_vector));

    // Test Tokenization
    println!("****** Tokenization Tests ******");

}
