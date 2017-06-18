#[derive(Debug)]
pub enum Token {
    Char(char),
    BeginParen,      // '('
    EndParen,        // ')'
    Asterisk,        // '*'
    Pipe,            // '|'
}

pub type Tokens = Vec<Token>;

pub fn tokenize(s: String) -> Tokens {
    let tokenize_char = |c| {
        match c {
            '(' => Token::BeginParen,
            ')' => Token::EndParen,
            '*' => Token::Asterisk,
            '|' => Token::Pipe,
            _   => Token::Char(c),
        }
    };
    s.chars().map(tokenize_char).collect::<Vec<_>>()
}



