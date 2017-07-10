#![feature(box_syntax, box_patterns)]
mod token;
mod parser;
mod vm;

#[test]
fn parser_test() {
    let test_case = vec![
        "a",
        "aa",
        "(a)",
        "a*a",
        "a*b*",
        "(a*)c*",
        "(a*b)c*",
        "(a*b*)c*",
    ];
    for t in test_case {
        let code = token::tokenize(String::from(t));
        let ast = parser::parse(&mut code.iter().peekable());
        println!("{:?}", t);
        println!("{:?}", ast);
    }
}


fn main() {
    let code1 = token::tokenize(String::from("a*b*|c*|j"));
    let ast = parser::parse(&mut code1.iter().peekable());
    let ir = vm::compile(ast.unwrap());
    // println!("{:?}", code1);
    // println!("{:?}", ast);
    vm::dump(ir);
}
