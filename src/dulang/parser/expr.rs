/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::lexer::token::Token;
use crate::dulang::parser::type_spec::TypeSpec;

pub enum Expr {
    IntExpr {
        op: Token,
        value: i32,
    },
    FloatExpr {
        op: Token,
        value: f64,
    },
    StringExpr {
        op: Token,
        value: String,
    },
    NameExpr {
        op: Token,
        name: String,
    },
    CompoundExpr {
        op: Token,
        elem_type: TypeSpec,
        num_args: usize,
        args: Vec<Expr>,
    },
    CastExpr {
        op: Token,
        cast_type: TypeSpec,
        expr: Expr,
    },
    UnaryExpr {
        op: Token,
        operand: Expr,
    },
    BinaryExpr {
        op: Token,
        left: Expr,
        right: Expr,
    },
    TernaryExpr {
        op: Token,
        condition: Expr,
        then_expr: Expr,
        else_expr: Expr,
    },
    CallExpr {
        op: Token,
        expr: Expr,
        num_args: Expr,
        args: Vec<Expr>,
    },
    IndexExpr {
        op: Token,
        expr: Expr,
        index: Expr,
    },
    FieldExpr {
        op: Token,
        expr: Expr,
        name: String,
    },
}
