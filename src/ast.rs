use big_num::BigNum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AstNode {
    Literal(BigNum),
    Add(Box<AstNode>, Box<AstNode>),
    Sub(Box<AstNode>, Box<AstNode>),
    Mul(Box<AstNode>, Box<AstNode>),
    Div(Box<AstNode>, Box<AstNode>),
    Pow(Box<AstNode>, Box<AstNode>),
    Mod(Box<AstNode>, Box<AstNode>),
}
