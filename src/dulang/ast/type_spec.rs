/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::expr::Expr;
use crate::dulang::ast::type_spec::TypeSpec::{
    ArrayTypeSpec, FuncTypeSpec, NameTypeSpec, PtrTypeSpec,
};
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeSpec {
    NameTypeSpec {
        name_spec: String,
    },
    FuncTypeSpec {
        num_args: usize,
        args_type: Vec<Box<TypeSpec>>,
        ret_type: Box<TypeSpec>,
    },
    ArrayTypeSpec {
        size: Box<Expr>,
        elem_type: Box<TypeSpec>,
    },
    PtrTypeSpec {
        ptr_type: Box<TypeSpec>,
    },
}

impl Display for TypeSpec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            NameTypeSpec { ref name_spec } => {
                return f.write_str(&format!("NameTypeSpec({})", name_spec));
            }
            FuncTypeSpec {
                ref num_args,
                ref args_type,
                ref ret_type,
            } => {
                return f.write_str(&format!("NameTypeSpec({} {:?})", ret_type, args_type));
            }
            ArrayTypeSpec {
                ref size,
                ref elem_type,
            } => {
                return f.write_str(&format!("ArrayTypeSpec({})", elem_type));
            }
            PtrTypeSpec { ref ptr_type } => {
                return f.write_str(&format!("PtrTypeSpec({})", ptr_type));
            }
            _ => {
                return f.write_str(&format!("Unknown TypeSpec"));
            }
        }
    }
}
