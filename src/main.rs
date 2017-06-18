#![feature(box_syntax, box_patterns)]

mod token;

// #[derive(Debug)]
// enum Ast {
//     Concat(Box<Ast>, Box<Ast>),
//     Alt(Box<Ast>, Box<Ast>),
//     Star(Box<Ast>),
//     Char(char),
// }
//
// fn eat(code: &mut token::Tokens) -> token::Tokens {
//     (code, head) = code.split_off(1);
//     head
// }
//
// fn parse_expr(code: &mut token::Tokens) -> Result<Ast, &'static str> {
//     //term ("|" term)*
//     
//     let mut ast = match code.peek() {//FIXME
//         Char        => parse_term(code),
//         BeginParen  => parse_term(code),
//         _           => Err("Error: parse_term"),
//     }.unwrap();
//     match code.peek() { //FIXME
//         Token::Pipe => concat_ast(box ast, box parse_expr(code)),
//         _           => Err("Error: parse_term"),
//     }.unwrap()
// }
//
// fn parse_term(code: &mut token::Tokens) -> Result<Ast, &'static str> {
//     //factor*
//     let mut ast = parse_factor(code).unwrap();
//     let head_next = code.peek(); //FIXME
//     match head_next {
//         Token::Asterisk => Ok(Ast::Star(box ast)),
//         _               => Ok(ast),
//     }
// }
//
// fn parse_factor(code: &mut token::Tokens) -> Result<Ast, &'static str> {
//     //Char | Char*
//     let mut ast = parse_char(code).unwrap();
//     match code.peek() {
//         Token::Asterisk => Ok(Ast::Star(box ast)),
//         _               => Ok(ast),
//     }
// }
//
// fn parse_char(code: &mut token::Tokens) -> Result<Ast, &'static str> {
//     //str | (expr)
//     match code.peek() { //FIXME
//         Token::Char(x)      => Ok(Ast::Char(x)),
//         Token::BeginParen   => {
//             let _ast = parse_expr(code);
//             eat(code, Token::EndParen);
//             _ast
//         }
//         _                   => {
//             Err("Error: parse_factor");
//         },
//     }
// }

fn main() {
    let code = token::tokenize(String::from("(a*b*)*"));
    println!("code: {:?}", code);
    // let ans = expr(code);
}
