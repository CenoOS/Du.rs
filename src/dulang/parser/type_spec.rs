/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::parser::expr::Expr;

pub enum TypeSpec {
    FuncTypeSpec {
        num_args: usize,
        args_type: Vec<TypeSpec>,
        ret_type: TypeSpec,
    },
    ArrayTypeSpec {
        size: Expr,
        elem_type: TypeSpec,
    },
    PtrTypeSpec {
        ptr_type: TypeSpec,
    },
}
