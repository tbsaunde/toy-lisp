use std::boxed::Box;

pub mod exp;

#[derive(Debug)]
pub enum ASTNode {
    Plus(Box<ASTNode>, Box<ASTNode>),
    Minus(Box<ASTNode>, Box<ASTNode>),
    Num(i32),
    Symbol(String),
    Definition(Box<ASTNode>, Box<ASTNode>),
    Subst(Box<ASTNode>, Box<ASTNode>),
    Nil(),
    List(Box<ASTNode>, Box<ASTNode>),
    Head(Box<ASTNode>),
    Tail(Box<ASTNode>),
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("invalid arguments");
        return;
    }

    let exp = exp::parse_Expr(&args[1]);
    println!("{:?}", exp);
}
