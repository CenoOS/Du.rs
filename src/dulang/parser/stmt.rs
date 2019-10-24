/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::lexer::token::Token;
use crate::dulang::parser::expr::Expr;

pub struct StmtBlock {
    num_stmts: usize,
    stmts: Vec<Stmt>,
}

pub struct ElseIfStmt {
    condition: Expr,
    block: StmtBlock,
}

pub struct SwitchCaseStmt {
    num_expr: usize,
    expr: Vec<Expr>,
    is_default: bool,
    block: StmtBlock,
}

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
    StmtBlock {
        stmt_block: StmtBlock,
    },
}
