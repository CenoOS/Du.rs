/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::decl::Decl::{
    AggregateDecl, ConstDecl, EnumDecl, FuncDecl, TypedefDecl, VarDecl,
};
use crate::dulang::ast::expr::Expr;
use crate::dulang::ast::stmt::StmtBlock;
use crate::dulang::ast::type_spec::TypeSpec;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumItem {
    name: String,
    type_spec: TypeSpec,
}

impl Display for EnumItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!("EnumItem({} {})", self.name, self.type_spec));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AggregateItem {
    name: Vec<String>,
    num_names: usize,
    type_spec: TypeSpec,
}

impl Display for AggregateItem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!(
            "AggregateItem({:?} {})",
            self.name, self.type_spec
        ));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncParam {
    name: String,
    type_spec: TypeSpec,
}

impl Display for FuncParam {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        return f.write_str(&format!("FuncParam({} {})", self.name, self.type_spec));
    }
}

#[derive(Debug, Clone, PartialEq)]
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

impl Display for Decl {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            EnumDecl {
                ref name,
                ref items,
                ref num_items,
            } => {
                return f.write_str(&format!("EnumDecl({} {:?})", name, items));
            }
            AggregateDecl {
                ref name,
                ref items,
                ref num_items,
            } => {
                return f.write_str(&format!("AggregateDecl({} {:?})", name, items));
            }
            FuncDecl {
                ref name,
                ref params,
                ref num_params,
                ref return_type,
                ref block,
            } => {
                return f.write_str(&format!("FuncDecl({:?} {} {})", params, return_type, block));
            }
            TypedefDecl {
                ref name,
                ref type_spec,
            } => {
                return f.write_str(&format!("TypedefDecl({})", type_spec));
            }
            VarDecl {
                ref name,
                ref type_spec,
                ref expr,
            } => {
                return f.write_str(&format!("VarDecl({} {:?} {:?})", name, type_spec, expr));
            }
            ConstDecl { ref name, ref expr } => {
                return f.write_str(&format!("ConstDecl({} {})", name, expr));
            }
            _ => {
                return f.write_str(&format!("Unknown Decl"));
            }
        }
    }
}
