/*
 * Copyright (c) 2019. NeroYang
 */

use crate::dulang::ast::expr::Expr;

pub enum TypeSpec {
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
