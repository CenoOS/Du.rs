/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dolang::ast::expr::Expr;
use crate::dolang::ast::stmt::Stmt::{
    AssignStmt, BlockStmt, ForStmt, IfStmt, InitStmt, ReturnStmt, SwitchStmt, WhileStmt,
};
use crate::dolang::lexer::token::Token;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct StmtBlock {
    num_stmts: usize,
    stmts: Vec<Stmt>,
}

impl Display for StmtBlock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!("StmtBlock({:?})", self.stmts));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfStmt {
    condition: Expr,
    block: StmtBlock,
}

impl Display for ElseIfStmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!("ElseIfStmt({} {})", self.condition, self.block));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwitchCaseStmt {
    num_expr: usize,
    expr: Vec<Expr>,
    is_default: bool,
    block: StmtBlock,
}

impl Display for SwitchCaseStmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!(
            "SwitchCaseStmt({:?} {} {})",
            self.expr, self.is_default, self.block
        ));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    ReturnStmt {
        expr: Expr,
    },
    IfStmt {
        condition: Expr,
        then_block: StmtBlock,
        else_ifs: Vec<ElseIfStmt>,
        num_else_ifs: usize,
        else_block: StmtBlock,
    },
    WhileStmt {
        condition: Expr,
        block: StmtBlock,
    },
    ForStmt {
        init: Box<Stmt>,
        condition: Expr,
        next: Box<Stmt>,
        block: StmtBlock,
    },
    SwitchStmt {
        num_cases: usize,
        expr: Expr,
        cases: Vec<SwitchCaseStmt>,
    },
    AssignStmt {
        op: Token,
        left: Expr,
        right: Expr,
    },
    InitStmt {
        var_name: String,
        expr: Expr,
    },
    Expr {
        expr: Expr,
    },
    BlockStmt {
        stmt_block: StmtBlock,
    },
}

impl Display for Stmt {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ReturnStmt { ref expr } => {
                return f.write_str(&format!("ReturnStmt({})", expr));
            }
            IfStmt {
                ref condition,
                ref then_block,
                ref else_ifs,
                num_else_ifs,
                ref else_block,
            } => {
                return f.write_str(&format!(
                    "IfStmt({} {} {:?} {})",
                    condition, then_block, else_ifs, else_block
                ));
            }
            WhileStmt {
                ref condition,
                ref block,
            } => {
                return f.write_str(&format!("WhileStmt({} {})", condition, block));
            }
            ForStmt {
                ref init,
                ref condition,
                ref next,
                ref block,
            } => {
                return f.write_str(&format!(
                    "WhileStmt({} {} {} {})",
                    init, condition, next, block
                ));
            }
            SwitchStmt {
                num_cases,
                ref expr,
                ref cases,
            } => {
                return f.write_str(&format!("SwitchStmt({} {:?})", expr, cases));
            }
            AssignStmt {
                ref op,
                ref left,
                ref right,
            } => {
                return f.write_str(&format!("AssignStmt({} {} {})", op, left, right));
            }
            InitStmt {
                ref var_name,
                ref expr,
            } => {
                return f.write_str(&format!("InitStmt({} {})", var_name, expr));
            }
            BlockStmt { ref stmt_block } => {
                return f.write_str(&format!("BlockStmt({})", stmt_block));
            }
            _ => {
                return f.write_str(&format!("Unknown Stmt"));
            }
        }
    }
}
