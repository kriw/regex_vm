use std::iter::Peekable;
use token::Token;

#[derive(Debug)]
pub enum Ast {
    Concat(Box<Ast>, Box<Ast>),
    Alt(Box<Ast>, Box<Ast>),
    Star(Box<Ast>),
    Char(char),
}

fn concat_ast(ast1: Box<Ast>, ast2: Box<Ast>) -> Ast {
    Ast::Concat(ast1, ast2)
}

fn eat<'a, T>(code: &mut Peekable<T>, token: Token) 
    where T: Iterator<Item=&'a Token> {

    let head = code.next().unwrap();
    assert_eq!(*head, token);
}

fn parse_expr<'a, T>(code: &mut Peekable<T>) -> Result<Ast, &'static str>
    where T: Iterator<Item=&'a Token> {

    let mut ast = match **code.peek().unwrap() {
        Token::Char(_)     => parse_term(code),
        Token::BeginParen  => parse_term(code),
        _                  => Err("Error: parse_expr"),
    };
    while code.peek().is_some() {
        match **code.peek().unwrap() {
            Token::Pipe => {
                eat(code, Token::Pipe);
                let right = parse_expr(code).unwrap();
                ast = Ok(Ast::Alt(box ast.unwrap(), box right));
            },
            _           => break,
        };
    };
    return ast;
}

fn parse_term<'a, T>(code: &mut Peekable<T>) -> Result<Ast, &'static str>
    where T: Iterator<Item=&'a Token> {

    macro_rules! po {
        ($ast:expr) => {
            $ast = concat_ast(box $ast, box parse_factor(code).unwrap());
        }
    };
    let mut ast = parse_factor(code).unwrap();
    while code.peek().is_some() {
        match **code.peek().unwrap() {
            Token::Char(_) => po!(ast),
            Token::BeginParen => po!(ast),
            _       => {
                return Ok(ast);
            }
        };
    }
    return Ok(ast);
}

fn parse_factor<'a, T>(code: &mut Peekable<T>) -> Result<Ast, &'static str>
    where T: Iterator<Item=&'a Token> {

    let ast = parse_char(code).unwrap();
    if code.peek().is_none() {
        return Ok(ast);
    }
    match **code.peek().unwrap() {
        Token::Asterisk => {
            eat(code, Token::Asterisk);
            Ok(Ast::Star(box ast))
        },
        _               => Ok(ast),
    }
}

fn parse_char<'a, T>(code: &mut Peekable<T>) -> Result<Ast, &'static str>
    where T: Iterator<Item=&'a Token> {

    let ast = match **code.peek().unwrap() {
        Token::Char(x)      => {
            eat(code, Token::Char(x));
            Ok(Ast::Char(x))
        },
        Token::BeginParen   => {
            eat(code, Token::BeginParen);
            let _ast = parse_expr(code);
            eat(code, Token::EndParen);
            _ast
        },
        _                   => Err("Error: parse_char"),
    }.unwrap();
    return Ok(ast);
}

pub fn parse<'a, T>(code: &mut Peekable<T>) -> Result<Ast, &'static str>
    where T: Iterator<Item=&'a Token> {
    
    return parse_expr(code);
}
