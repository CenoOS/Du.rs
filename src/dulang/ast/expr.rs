/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::type_spec::TypeSpec;
use crate::dulang::lexer::int::Int;
use crate::dulang::lexer::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    IntExpr {
        value: Int,
    },
    FloatExpr {
        value: f64,
    },
    StringExpr {
        value: String,
    },
    NameExpr {
        name: String,
    },
    CompoundExpr {
        op: Token,
        elem_type: TypeSpec,
        num_args: usize,
        args: Vec<Box<Expr>>,
    },
    CastExpr {
        op: Token,
        cast_type: TypeSpec,
        expr: Box<Expr>,
    },
    UnaryExpr {
        op: Token,
        operand: Box<Expr>,
    },
    BinaryExpr {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    TernaryExpr {
        condition: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
    CallExpr {
        expr: Box<Expr>,
        num_args: usize,
        args: Vec<Box<Expr>>,
    },
    IndexExpr {
        expr: Box<Expr>,
        index: Box<Expr>,
    },
    FieldExpr {
        expr: Box<Expr>,
        name: String,
    },
}
