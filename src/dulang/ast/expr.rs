/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::expr::Expr::{
    BinaryExpr, CallExpr, CastExpr, CompoundExpr, FieldExpr, FloatExpr, IndexExpr, IntExpr,
    NameExpr, StringExpr, TernaryExpr, UnaryExpr,
};
use crate::dulang::ast::type_spec::TypeSpec;
use crate::dulang::lexer::int::Int;
use crate::dulang::lexer::token::Token;
use std::fmt;
use std::fmt::{Display, Formatter};

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

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            IntExpr { ref value } => {
                return f.write_str(&format!("IntExpr({})", value));
            }
            FloatExpr { ref value } => {
                return f.write_str(&format!("FloatExpr({})", value));
            }
            NameExpr { ref name } => {
                return f.write_str(&format!("NameExpr({})", name));
            }
            StringExpr { ref value } => {
                return f.write_str(&format!("StringExpr({})", value));
            }
            CompoundExpr {
                ref op,
                ref elem_type,
                ref num_args,
                ref args,
            } => {
                return f.write_str(&format!("CompoundExpr({} {} {:?})", op, elem_type, args));
            }
            CastExpr {
                ref op,
                ref cast_type,
                ref expr,
            } => {
                return f.write_str(&format!("CastExpr({} {} {})", op, cast_type, expr));
            }
            UnaryExpr {
                ref op,
                ref operand,
            } => {
                return f.write_str(&format!("UnaryExpr({} {})", op, operand));
            }
            BinaryExpr {
                ref op,
                ref left,
                ref right,
            } => {
                return f.write_str(&format!("BinaryExpr({} {} {})", op, left, right));
            }
            TernaryExpr {
                ref condition,
                ref then_expr,
                ref else_expr,
            } => {
                return f.write_str(&format!(
                    "TernaryExpr({} {} {})",
                    condition, then_expr, else_expr
                ));
            }
            CallExpr {
                ref expr,
                ref num_args,
                ref args,
            } => {
                return f.write_str(&format!("CallExpr({} {:?})", expr, args));
            }
            IndexExpr {
                ref expr,
                ref index,
            } => {
                return f.write_str(&format!("IndexExpr({} {})", expr, index));
            }
            FieldExpr { ref expr, ref name } => {
                return f.write_str(&format!("FieldExpr({} {})", expr, name));
            }
            _ => {
                return f.write_str(&format!("Unknown Expr"));
            }
        }
    }
}
