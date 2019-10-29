/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::expr::Expr;
use crate::dulang::ast::stmt::StmtBlock;
use crate::dulang::ast::type_spec::TypeSpec;

pub struct EnumItem {
    name: String,
    type_spec: TypeSpec,
}

pub struct AggregateItem {
    name: Vec<String>,
    num_names: usize,
    type_spec: TypeSpec,
}

pub struct FuncParam {
    name: String,
    type_spec: TypeSpec,
}

pub enum Decl {
    EnumDecl {
        name: String,
        items: Vec<EnumItem>,
        num_items: usize,
    },
    AggregateDecl {
        name: String,
        items: Vec<AggregateItem>,
        num_items: usize,
    },
    FuncDecl {
        name: String,
        params: Vec<FuncParam>,
        num_params: usize,
        return_type: TypeSpec,
        block: StmtBlock,
    },
    TypedefDecl {
        name: String,
        type_spec: TypeSpec,
    },
    VarDecl {
        name: String,
        type_spec: Option<TypeSpec>,
        expr: Option<Expr>,
    },
    ConstDecl {
        name: String,
        expr: Expr,
    },
}
