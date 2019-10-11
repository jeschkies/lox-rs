use crate::token::Token;

pub enum Expr {
    Binary { left: Expr, operator: Token, right: Expr },
    Grouping {expression: Expr },
    Literal { value: String }, // Object in chapter 5
    Unaary { operator: Token, right: Expr },
}