/*
 * Copyright (c) 2019. NeroYang
 */
use crate::assembler::assembler_error::AssemblerError::ParseError;
use crate::dolang::ast::decl::Decl;
use crate::dolang::ast::decl::Decl::{ConstDecl, VarDecl};
use crate::dolang::ast::expr::Expr;
use crate::dolang::ast::expr::Expr::{
    BinaryExpr, CallExpr, FieldExpr, FloatExpr, IndexExpr, IntExpr, NameExpr, StringExpr,
    TernaryExpr, UnaryExpr,
};
use crate::dolang::ast::type_spec::TypeSpec;
use crate::dolang::ast::type_spec::TypeSpec::{
    ArrayTypeSpec, FuncTypeSpec, NameTypeSpec, PtrTypeSpec,
};
use crate::dolang::lexer::keyword::Keyword::{
    KeywordConst, KeywordEnum, KeywordFunc, KeywordGoto, KeywordImport, KeywordStruct,
    KeywordTypeDef, KeywordVar,
};
use crate::dolang::lexer::lexer::Lexer;
use crate::dolang::lexer::token::Token;
use crate::dolang::lexer::token::Token::{
    TokenAdd, TokenAddAssign, TokenAndAssign, TokenAssign, TokenBand, TokenBor, TokenColon,
    TokenColonAssign, TokenComma, TokenDec, TokenDiv, TokenDivAssign, TokenDot, TokenEqual,
    TokenFloat, TokenGreaterThan, TokenGreaterThanEqual, TokenHashTag, TokenInc, TokenInt,
    TokenKeyword, TokenLeftBrackets, TokenLeftCurlyBrackets, TokenLeftShift, TokenLeftShiftAssign,
    TokenLeftSquareBrackets, TokenLessThan, TokenLessThanEqual, TokenMod, TokenModAssign, TokenMul,
    TokenMulAssign, TokenName, TokenNot, TokenNotEqual, TokenOr, TokenOrAssign, TokenQuestionMark,
    TokenRightBrackets, TokenRightShift, TokenRightShiftAssign, TokenRightSquareBrackets,
    TokenSemiColon, TokenStr, TokenSub, TokenSubAssign, TokenXor, TokenXorAssign,
};
use crate::dolang::parser::parser_error::ParserError;
use crate::dolang::parser::parser_error::ParserError::UnexpectedTokenError;
use crate::vm::instruction::OpCode::POP;
use std::process::exit;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Result<Token, &'static str>,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub(crate) fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            current_token: Ok(TokenHashTag {}),
            errors: Vec::new(),
        }
    }

    fn is_token(&self, expected_token: Token) -> bool {
        match self.current_token {
            Ok(ref token) => {
                if *token == expected_token {
                    return true;
                }
                return false;
            }
            _ => {
                return false;
            }
        }
    }

    fn expect_token(&mut self, expected_token: Token) {
        match self.current_token {
            Ok(ref token) => {
                if *token != expected_token {
                    panic!("SyntaxError: expect token :{}", expected_token);
                }
            }
            _ => {
                panic!("SyntaxError: expect token :{}", expected_token);
            }
        }
    }

    pub(crate) fn match_token(&mut self, expected_token: Token) -> bool {
        let token = self.next_token();
        match token {
            Ok(ref token) => {
                if *token == expected_token {
                    return true;
                }
                return false;
            }
            Err(e) => {
                return false;
            }
        }
    }

    fn is_cmp_op(&self, token: &Token) -> bool {
        return match token {
            TokenEqual {}
            | TokenNotEqual {}
            | TokenLessThan {}
            | TokenGreaterThan {}
            | TokenGreaterThanEqual {}
            | TokenLessThanEqual {} => true,
            _ => false,
        };
    }

    fn is_add_op(&self, token: &Token) -> bool {
        return match token {
            TokenAdd {} | TokenSub {} | TokenXor {} | TokenBor {} => true,
            _ => false,
        };
    }

    fn is_mul_op(&self, token: &Token) -> bool {
        return match token {
            TokenMul {}
            | TokenMod {}
            | TokenDiv {}
            | TokenBand {}
            | TokenLeftShift {}
            | TokenRightShift {} => true,
            _ => false,
        };
    }

    pub(crate) fn is_unary_op(&self, token: &Token) -> bool {
        return match token {
            TokenInc {}
            | TokenDec {}
            | TokenMul {}
            | TokenSub {}
            | TokenNot {}
            | TokenBand {}
            | TokenSub {}
            | TokenAdd {} => true,
            _ => false,
        };
    }

    fn is_token_left_bracket(&self, token: &Token) -> bool {
        return match token {
            TokenLeftBrackets {} => true,
            _ => false,
        };
    }

    fn is_token_left_square_bracket(&self, token: &Token) -> bool {
        return match token {
            TokenLeftSquareBrackets {} => true,
            _ => false,
        };
    }

    fn is_token_dot(&self, token: &Token) -> bool {
        return match token {
            TokenDot {} => true,
            _ => false,
        };
    }

    fn is_token_comma(&self, token: &Token) -> bool {
        return match token {
            TokenComma {} => true,
            _ => false,
        };
    }

    fn is_assign_op(&self, token: &Token) -> bool {
        return match token {
            TokenAssign {}
            | TokenColonAssign {}
            | TokenAddAssign {}
            | TokenSubAssign {}
            | TokenAndAssign {}
            | TokenOrAssign {}
            | TokenXorAssign {}
            | TokenMulAssign {}
            | TokenDivAssign {}
            | TokenModAssign {}
            | TokenLeftShiftAssign {}
            | TokenRightShiftAssign {} => true,
            _ => false,
        };
    }

    pub(crate) fn next_token(&mut self) -> Result<Token, &'static str> {
        let token = self.lexer.next_token();
        self.current_token = token.clone();
        return token;
    }

    fn parse_type_func_param(&mut self) -> Option<TypeSpec> {
        let mut type_spec = self.parse_type_spec();
        if self.match_token(TokenColon {}) {
            if type_spec.unwrap().is_name() {
                self.errors.push(UnexpectedTokenError {
                    token: self.current_token.clone().unwrap(),
                    line: 0,
                });
                return None;
            }
            type_spec = self.parse_type_spec();
        }
        return type_spec;
    }

    fn parse_type_func(&mut self) -> Option<TypeSpec> {
        self.match_token(TokenLeftSquareBrackets {});
        let mut args: Vec<Box<TypeSpec>> = Vec::new();
        if !self.is_token(TokenRightSquareBrackets {}) {
            args.push(Box::new(self.parse_type_func_param().unwrap()));
            while self.match_token(TokenComma {}) {
                args.push(Box::new(self.parse_type_func_param().unwrap()));
            }
        }
        self.match_token(TokenRightSquareBrackets {});
        let mut ret = None;
        if self.match_token(TokenColon {}) {
            ret = self.parse_type_spec();
        }

        return Some(FuncTypeSpec {
            num_args: args.len(),
            args_type: args,
            ret_type: Box::new(ret.unwrap()),
        });
    }

    fn parse_type_base(&mut self) -> Option<TypeSpec> {
        match self.current_token.clone().unwrap() {
            TokenName { ref name } => {
                return Some(NameTypeSpec {
                    name_spec: name.to_string(),
                });
            }
            TokenKeyword { ref keyword } => match keyword {
                KeywordFunc { ref name } => {
                    return self.parse_type_func();
                }
                _ => None,
            },
            TokenRightSquareBrackets {} => {
                return self.parse_type_spec();
            }
            _ => {
                self.errors.push(UnexpectedTokenError {
                    token: self.current_token.clone().unwrap(),
                    line: 0,
                });
                return None;
            }
        }
    }

    pub(crate) fn parse_type_spec(&mut self) -> Option<TypeSpec> {
        self.next_token();
        let mut type_spec = self.parse_type_base();
        self.next_token();
        while self.is_token(TokenLeftSquareBrackets {}) || self.is_token(TokenMul {}) {
            match self.current_token {
                Ok(ref token) => match token {
                    TokenLeftSquareBrackets {} => {
                        let mut expr = None;
                        self.next_token();
                        if (self.current_token.clone().unwrap() != TokenRightSquareBrackets {}) {
                            expr = self.parse_expr();
                        }
                        self.match_token(TokenRightSquareBrackets {});
                        type_spec = Some(ArrayTypeSpec {
                            size: Box::new(expr.unwrap()),
                            elem_type: Box::new(type_spec.unwrap()),
                        });
                    }
                    TokenMul {} => {
                        self.next_token();
                        type_spec = self.parse_type_base();
                        type_spec = Some(PtrTypeSpec {
                            ptr_type: Box::new(type_spec.unwrap()),
                        });
                    }
                    _ => {
                        return None;
                    }
                },
                _ => {
                    return None;
                }
            }
        }
        return type_spec;
    }

    fn parse_expr_compound(&mut self, _type_spec: Option<TypeSpec>) -> Option<Expr> {
        return None;
    }

    pub(crate) fn parse_expr_operand(&mut self) -> Option<Expr> {
        let token = self.current_token.clone();
        match token {
            Ok(ref token) => match token {
                TokenName { name } => {
                    self.next_token();
                    match token {
                        TokenLeftCurlyBrackets {} => {
                            return self.parse_expr_compound(Some(NameTypeSpec {
                                name_spec: name.to_string(),
                            }));
                        }
                        _ => {
                            return Some(NameExpr {
                                name: name.to_string(),
                            });
                        }
                    }
                }
                TokenInt { value } => {
                    self.next_token();
                    return Some(IntExpr {
                        value: value.clone(),
                    });
                }
                TokenFloat { value } => {
                    return Some(FloatExpr { value: *value });
                }
                TokenStr { value } => {
                    return Some(StringExpr {
                        value: value.to_string(),
                    });
                }
                TokenLeftCurlyBrackets {} => {
                    return self.parse_expr_compound(None);
                }
                TokenLeftBrackets {} => {
                    return None;
                }
                _ => {
                    self.errors.push(UnexpectedTokenError {
                        token: self.current_token.clone().unwrap(),
                        line: 0,
                    });
                    return None;
                }
            },
            _ => {
                self.errors.push(UnexpectedTokenError {
                    token: self.current_token.clone().unwrap(),
                    line: 0,
                });
                return None;
            }
        }
    }

    fn parse_expr_base(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_operand();
        while self.is_token_left_bracket(&self.current_token.clone().unwrap())
            || self.is_token_left_square_bracket(&self.current_token.clone().unwrap())
            || self.is_token_dot(&self.current_token.clone().unwrap())
        {
            if self.is_token(TokenLeftBrackets {}) {
                let mut args = Vec::new();
                args.push(Box::new(self.parse_expr().unwrap()));
                while self.is_token_comma(&self.current_token.clone().unwrap()) {
                    args.push(Box::new(self.parse_expr().unwrap()));
                }
                self.expect_token(TokenRightBrackets {});
                expr = Some(CallExpr {
                    expr: Box::new(expr.unwrap()),
                    num_args: args.len(),
                    args,
                })
            } else if self.is_token(TokenLeftSquareBrackets {}) {
                let index = self.parse_expr();
                self.expect_token(TokenRightSquareBrackets {});
                expr = Some(IndexExpr {
                    expr: Box::new(expr.unwrap()),
                    index: Box::new(index.unwrap()),
                });
            } else {
                let _token = self.next_token();
                match &self.current_token {
                    Ok(token) => match token {
                        TokenName { name } => {
                            expr = Some(FieldExpr {
                                expr: Box::new(expr.unwrap()),
                                name: name.to_string(),
                            })
                        }
                        _ => {
                            self.errors.push(UnexpectedTokenError {
                                token: self.current_token.clone().unwrap(),
                                line: 0,
                            });
                            return None;
                        }
                    },
                    _ => {
                        self.errors.push(UnexpectedTokenError {
                            token: self.current_token.clone().unwrap(),
                            line: 0,
                        });
                        return None;
                    }
                }
            }
        }
        return expr;
    }

    fn parse_expr_unary(&mut self) -> Option<Expr> {
        let mut token = self.current_token.clone().unwrap();
        if self.is_unary_op(&token) {
            let curr_token = self.current_token.clone().unwrap();
            token = self.next_token().unwrap();
            let right = self.parse_expr_unary().unwrap();
            return Some(UnaryExpr {
                op: curr_token,
                operand: Box::new(right),
            });
        } else {
            return self.parse_expr_base();
        }
    }

    fn parse_expr_mul(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_unary();
        let mut token = self.current_token.clone().unwrap();
        while self.is_mul_op(&token) {
            let curr_token = self.current_token.clone().unwrap();
            token = self.next_token().unwrap();
            let right = self.parse_expr_mul().unwrap();
            expr = Some(BinaryExpr {
                op: curr_token,
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    pub(crate) fn parse_expr_add(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_mul();
        let mut token = self.current_token.clone().unwrap();
        while self.is_add_op(&token) {
            let curr_token = self.current_token.clone().unwrap();
            token = self.next_token().unwrap();
            let right = self.parse_expr_mul().unwrap();
            expr = Some(BinaryExpr {
                op: curr_token,
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    pub(crate) fn parse_expr_cmp(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_add();
        let mut token = self.current_token.clone().unwrap();
        while self.is_cmp_op(&token) {
            let curr_token = self.current_token.clone().unwrap();
            token = self.next_token().unwrap();
            let right = self.parse_expr_add().unwrap();
            expr = Some(BinaryExpr {
                op: curr_token,
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn parse_expr_and(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_cmp();
        while self.is_token(TokenAdd {}) {
            expr = Some(BinaryExpr {
                op: TokenAdd {},
                left: Box::new(expr.unwrap()),
                right: Box::new(self.parse_expr_cmp().unwrap()),
            });
        }
        return expr;
    }

    fn parse_expr_or(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_and();
        while self.is_token(TokenOr {}) {
            expr = Some(BinaryExpr {
                op: TokenOr {},
                left: Box::new(expr.unwrap()),
                right: Box::new(self.parse_expr_and().unwrap()),
            });
        }
        return expr;
    }

    fn parse_expr_ternary(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_or();
        if self.is_token(TokenQuestionMark {}) {
            let expr_then = self.parse_expr();
            self.expect_token(TokenColon {});
            let expr_else = self.parse_expr();

            expr = Some(TernaryExpr {
                condition: Box::new(expr.unwrap()),
                then_expr: Box::new(expr_then.unwrap()),
                else_expr: Box::new(expr_else.unwrap()),
            });
        }
        return expr;
    }

    pub(crate) fn parse_expr(&mut self) -> Option<Expr> {
        return self.parse_expr_ternary();
    }

    fn parse_name(&mut self) -> Option<String> {
        let token = self.next_token();
        match token {
            Ok(TokenName { name }) => {
                return Some(name.parse().unwrap());
            }
            _ => {
                let error = UnexpectedTokenError {
                    token: token.unwrap(),
                    line: 0,
                };
                self.errors.push(error);
                return None;
            }
        }
    }

    fn parse_decl_enum(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_type_def(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_struct(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_var(&mut self) -> Option<Decl> {
        let name = self.parse_name();
        let token = self.next_token();
        match token {
            Ok(token) => match token {
                TokenAssign {} => {
                    self.next_token();
                    let expr = self.parse_expr();
                    self.expect_token(TokenSemiColon {});
                    return Some(VarDecl {
                        name: name.unwrap(),
                        type_spec: None,
                        expr,
                    });
                }
                TokenColon {} => {
                    let type_spec = self.parse_type_spec();
                    let mut expr = None;
                    if self.is_token(TokenAssign {}) {
                        self.next_token();
                        expr = self.parse_expr();
                    }
                    self.expect_token(TokenSemiColon {});
                    return Some(VarDecl {
                        name: name.unwrap(),
                        type_spec,
                        expr,
                    });
                }
                _ => {
                    self.errors.push(UnexpectedTokenError { token, line: 0 });
                }
            },
            _ => {
                self.errors.push(UnexpectedTokenError {
                    token: token.unwrap(),
                    line: 0,
                });
            }
        }
        return None;
    }

    fn parse_decl_const(&mut self) -> Option<Decl> {
        let name = self.parse_name();
        let token = self.next_token();
        match token {
            Ok(token) => match token {
                TokenAssign {} => {
                    self.next_token();
                    let expr = self.parse_expr();
                    self.expect_token(TokenSemiColon {});
                    return Some(ConstDecl {
                        name: name.unwrap(),
                        type_spec: None,
                        expr,
                    });
                }
                TokenColon {} => {
                    let type_spec = self.parse_type_spec();
                    let mut expr = None;
                    if self.is_token(TokenAssign {}) {
                        self.next_token();
                        expr = self.parse_expr();
                    }
                    self.expect_token(TokenSemiColon {});
                    return Some(ConstDecl {
                        name: name.unwrap(),
                        type_spec,
                        expr,
                    });
                }
                _ => {
                    self.errors.push(UnexpectedTokenError { token, line: 0 });
                }
            },
            _ => {
                self.errors.push(UnexpectedTokenError {
                    token: token.unwrap(),
                    line: 0,
                });
            }
        }
        return None;
    }

    fn parse_decl_func(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_import(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_opt(&mut self) -> Option<Decl> {
        match self.next_token() {
            Ok(keyword) => match keyword {
                TokenKeyword { keyword } => match keyword {
                    KeywordVar { name } => {
                        return self.parse_decl_var();
                    }
                    KeywordConst { name } => {
                        return self.parse_decl_const();
                    }
                    KeywordEnum { name } => {
                        return self.parse_decl_enum();
                    }
                    KeywordTypeDef { name } => {
                        return self.parse_decl_type_def();
                    }
                    KeywordStruct { name } => {
                        return self.parse_decl_struct();
                    }
                    KeywordFunc { name } => {
                        return self.parse_decl_func();
                    }
                    KeywordImport { name } => {
                        return self.parse_decl_import();
                    }
                    _ => {}
                },
                Token => {}
            },
            _ => {
                return None;
                self.errors.push(UnexpectedTokenError {
                    token: self.current_token.clone().unwrap(),
                    line: 0,
                });
            }
        }
        return None;
    }

    pub(crate) fn parse_decl(&mut self) -> Option<Decl> {
        let decl = self.parse_decl_opt();
        return decl;
    }
}
