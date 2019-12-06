/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::dolang::ast::decl::Decl::{ConstDecl, VarDecl};
    use crate::dolang::ast::expr::Expr::{BinaryExpr, IntExpr, NameExpr, UnaryExpr};
    use crate::dolang::ast::type_spec::TypeSpec::{ArrayTypeSpec, NameTypeSpec, PtrTypeSpec};
    use crate::dolang::lexer::int::Int;
    use crate::dolang::lexer::int::Int::{IntBin, IntHex, IntOct};
    use crate::dolang::lexer::keyword::Keyword::KeywordVar;
    use crate::dolang::lexer::lexer::Lexer;
    use crate::dolang::lexer::token::Token::{
        TokenAdd, TokenAssign, TokenBand, TokenDec, TokenGreaterThan, TokenGreaterThanEqual,
        TokenInc, TokenInt, TokenKeyword, TokenLessThan, TokenLessThanEqual, TokenMul, TokenName,
        TokenSemiColon, TokenSub,
    };
    use crate::dolang::parser::parser::Parser;

    #[test]
    fn should_match_token() {
        let mut lexer = Lexer::new("let a = 1;");
        let mut parser = Parser::new(&mut lexer);
        assert_eq!(
            parser.match_token(TokenKeyword {
                keyword: KeywordVar {
                    name: "let".to_string()
                }
            }),
            true
        );
        assert_eq!(
            parser.match_token(TokenName {
                name: "a".to_string()
            }),
            true
        );
        assert_eq!(parser.match_token(TokenAssign {}), true);
        assert_eq!(
            parser.match_token(TokenInt {
                value: IntOct { value: 1 }
            }),
            true
        );
        assert_eq!(parser.match_token(TokenSemiColon {}), true);
    }

    #[test]
    fn should_return_true_when_token_is_unary() {
        let mut lexer = Lexer::new("");
        let parser = Parser::new(&mut lexer);
        assert_eq!(true, parser.is_unary_op(&TokenAdd {}));
        assert_eq!(true, parser.is_unary_op(&TokenSub {}));
        assert_eq!(true, parser.is_unary_op(&TokenMul {}));
        assert_eq!(true, parser.is_unary_op(&TokenBand {}));
    }

    #[test]
    fn should_parse_expr_operand() {
        let mut lexer = Lexer::new("a");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        assert_eq!(
            NameExpr {
                name: "a".to_string()
            },
            parser.parse_expr_operand().unwrap()
        );
    }

    #[test]
    fn should_parse_expr_operand_int() {
        let mut lexer = Lexer::new("120");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        assert_eq!(
            IntExpr {
                value: IntOct { value: 120 },
            },
            parser.parse_expr_operand().unwrap()
        );
    }

    #[test]
    fn should_parse_expr_operand_int_hex() {
        let mut lexer = Lexer::new("0x120");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        assert_eq!(
            IntExpr {
                value: IntHex { value: 288 },
            },
            parser.parse_expr_operand().unwrap()
        );
    }

    #[test]
    fn should_parse_add_expr() {
        let mut lexer = Lexer::new("a+b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_add();
        assert_eq!(
            decl.unwrap(),
            BinaryExpr {
                op: TokenAdd {},
                left: Box::new(NameExpr {
                    name: "a".to_string()
                }),
                right: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_cmp_gt_expr() {
        let mut lexer = Lexer::new("a > b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_cmp();
        assert_eq!(
            decl.unwrap(),
            BinaryExpr {
                op: TokenGreaterThan {},
                left: Box::new(NameExpr {
                    name: "a".to_string()
                }),
                right: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_cmp_lt_expr() {
        let mut lexer = Lexer::new("a < b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_cmp();
        assert_eq!(
            decl.unwrap(),
            BinaryExpr {
                op: TokenLessThan {},
                left: Box::new(NameExpr {
                    name: "a".to_string()
                }),
                right: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_cmp_gte_expr() {
        let mut lexer = Lexer::new("a >= b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_cmp();
        assert_eq!(
            decl.unwrap(),
            BinaryExpr {
                op: TokenGreaterThanEqual {},
                left: Box::new(NameExpr {
                    name: "a".to_string()
                }),
                right: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_cmp_lte_expr() {
        let mut lexer = Lexer::new("a <= b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_cmp();
        assert_eq!(
            decl.unwrap(),
            BinaryExpr {
                op: TokenLessThanEqual {},
                left: Box::new(NameExpr {
                    name: "a".to_string()
                }),
                right: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_inc_expr() {
        let mut lexer = Lexer::new("++b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_add();
        assert_eq!(
            decl.unwrap(),
            UnaryExpr {
                op: TokenInc {},
                operand: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_dec_expr() {
        let mut lexer = Lexer::new("--b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_add();
        assert_eq!(
            decl.unwrap(),
            UnaryExpr {
                op: TokenDec {},
                operand: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_neg_expr() {
        let mut lexer = Lexer::new("-b");
        let mut parser = Parser::new(&mut lexer);
        parser.next_token();
        let decl = parser.parse_expr_add();
        assert_eq!(
            decl.unwrap(),
            UnaryExpr {
                op: TokenSub {},
                operand: Box::new(NameExpr {
                    name: "b".to_string()
                }),
            }
        )
    }

    #[test]
    fn should_parse_var_decl() {
        let mut lexer = Lexer::new("let a = 1;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: Some(IntExpr {
                    value: IntOct { value: 1 }
                }),
            }
        );
    }

    #[test]
    fn should_parse_var_add_decl() {
        let mut lexer = Lexer::new("let a = 1+1;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(IntExpr {
                        value: IntOct { value: 1 }
                    }),
                    right: Box::new(IntExpr {
                        value: IntOct { value: 1 }
                    }),
                }),
            }
        );
    }

    #[test]
    fn should_parse_var_add_hex_binary_decl() {
        let mut lexer = Lexer::new("let a_b = 0x16 + 0b10101;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a_b".to_string(),
                type_spec: None,
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(IntExpr {
                        value: IntHex { value: 22 }
                    }),
                    right: Box::new(IntExpr {
                        value: IntBin { value: 21 }
                    }),
                }),
            }
        );
    }

    #[test]
    fn should_parse_var_add_variable_binary_decl() {
        let mut lexer = Lexer::new("let a = a + b;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(NameExpr {
                        name: "a".to_string(),
                    }),
                    right: Box::new(NameExpr {
                        name: "b".to_string()
                    }),
                }),
            }
        );
    }

    #[test]
    fn should_parse_type_spec() {
        let mut lexer = Lexer::new("int");
        let mut parser = Parser::new(&mut lexer);
        let type_spec = parser.parse_type_spec();
        assert_eq!(
            type_spec.unwrap(),
            NameTypeSpec {
                name_spec: "int".to_string()
            }
        )
    }

    #[test]
    fn should_parse_var_add_variable_binary_decl_with_type_spec() {
        let mut lexer = Lexer::new("let a:int = a + b;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: Some(NameTypeSpec {
                    name_spec: "int".to_string()
                }),
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(NameExpr {
                        name: "a".to_string(),
                    }),
                    right: Box::new(NameExpr {
                        name: "b".to_string()
                    }),
                }),
            }
        );
    }

    //    #[test]
    //    fn should_parse_var_add_variable_binary_decl_with_type_spec_ptr() {
    //        let mut lexer = Lexer::new("let a:*int = a + b;");
    //        let mut parser = Parser::new(&mut lexer);
    //        let decl = parser.parse_decl();
    //        assert_eq!(
    //            decl.unwrap(),
    //            VarDecl {
    //                name: "a".to_string(),
    //                type_spec: Some(PtrTypeSpec {
    //                    ptr_type: Box::new(NameTypeSpec {
    //                        name_spec: "int".to_string()
    //                    }),
    //                }),
    //                expr: Some(BinaryExpr {
    //                    op: TokenAdd {},
    //                    left: Box::new(NameExpr {
    //                        name: "a".to_string(),
    //                    }),
    //                    right: Box::new(NameExpr {
    //                        name: "b".to_string()
    //                    }),
    //                }),
    //            }
    //        );
    //    }

    #[test]
    fn should_parse_var_add_variable_binary_decl_with_type_spec_array() {
        let mut lexer = Lexer::new("let a:int[3] = a + b;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: Some(ArrayTypeSpec {
                    size: Box::from(IntExpr {
                        value: IntOct { value: 3 }
                    }),
                    elem_type: Box::from(NameTypeSpec {
                        name_spec: "int".to_string()
                    }),
                }),
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(NameExpr {
                        name: "a".to_string(),
                    }),
                    right: Box::new(NameExpr {
                        name: "b".to_string()
                    }),
                }),
            }
        );
    }

    //        #[test]
    //        fn should_parse_add_expr_decl() {
    //            let mut lexer = Lexer::new("c + d + e;");
    //            let mut parser = Parser::new(&mut lexer);
    //            let decl = parser.parse_expr();
    //            assert_eq!(
    //                decl.unwrap(),
    //                BinaryExpr {
    //                    op: TokenAdd {},
    //                    left: Box::new(BinaryExpr {
    //                        op: TokenAdd {},
    //                        left: Box::new(NameExpr {
    //                            name: "c".to_string()
    //                        }),
    //                        right: Box::new(NameExpr {
    //                            name: "d".to_string()
    //                        }),
    //                    }),
    //                    right: Box::new(NameExpr {
    //                        name: "e".to_string()
    //                    }),
    //                }
    //            );
    //        }
    //
    //    #[test]
    //    fn should_parse_var_add_variable_decl() {
    //        let mut lexer = Lexer::new("var a_b = c + d + e;");
    //        let mut parser = Parser::new(&mut lexer);
    //        let decl = parser.parse_decl();
    //        assert_eq!(
    //            decl.unwrap(),
    //            VarDecl {
    //                name: "a_b".to_string(),
    //                type_spec: None,
    //                expr: Some(BinaryExpr {
    //                    op: TokenAdd {},
    //                    left: Box::new(BinaryExpr {
    //                        op: TokenAdd {},
    //                        left: Box::new(NameExpr {
    //                            name: "c".to_string()
    //                        }),
    //                        right: Box::new(NameExpr {
    //                            name: "d".to_string()
    //                        }),
    //                    }),
    //                    right: Box::new(NameExpr {
    //                        name: "e".to_string()
    //                    }),
    //                }),
    //            }
    //        );
    //    }

    #[test]
    fn should_parse_const_decl() {
        let mut lexer = Lexer::new("const a = 1;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            ConstDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: Some(IntExpr {
                    value: IntOct { value: 1 }
                }),
            }
        );
    }

    #[test]
    fn should_parse_const_with_expr_decl() {
        let mut lexer = Lexer::new("const a = c+b;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            ConstDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: Some(BinaryExpr {
                    op: TokenAdd {},
                    left: Box::new(NameExpr {
                        name: "c".to_string(),
                    }),
                    right: Box::new(NameExpr {
                        name: "b".to_string()
                    }),
                }),
            }
        );
    }
}
