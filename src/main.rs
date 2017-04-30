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
    let exp = exp::parse_Expr("(+ 1 (- (HEAD (LIST 1 NIL)) 2))");
    println!("{:?}", exp);
    let exp = exp::parse_Expr("(HEAD (LIST 1 (TAIL (LIST 1 2))))");
    println!("{:?}", exp);
    let exp = exp::parse_Expr("SUBST x (DEFINE x 1)");
    println!("{:?}", exp);
}
