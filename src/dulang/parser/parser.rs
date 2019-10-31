/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::ast::decl::Decl;
use crate::dulang::ast::decl::Decl::VarDecl;
use crate::dulang::ast::expr::Expr;
use crate::dulang::ast::expr::Expr::{BinaryExpr, TernaryExpr, UnaryExpr};
use crate::dulang::ast::type_spec::TypeSpec;
use crate::dulang::lexer::keyword::Keyword::{
    KeywordConst, KeywordEnum, KeywordFunc, KeywordGoto, KeywordImport, KeywordStruct,
    KeywordTypeDef, KeywordVar,
};
use crate::dulang::lexer::lexer::Lexer;
use crate::dulang::lexer::token::Token;
use crate::dulang::lexer::token::Token::{
    TokenAdd, TokenAddAssign, TokenAndAssign, TokenAssign, TokenBand, TokenBor, TokenColon,
    TokenColonAssign, TokenDiv, TokenDivAssign, TokenEqual, TokenGreaterThan,
    TokenGreaterThanEqual, TokenHashTag, TokenKeyword, TokenLeftShift, TokenLeftShiftAssign,
    TokenLessThan, TokenLessThanEqual, TokenMod, TokenModAssign, TokenMul, TokenMulAssign,
    TokenName, TokenNotEqual, TokenOr, TokenOrAssign, TokenQuestionMark, TokenRightShift,
    TokenRightShiftAssign, TokenSemiColon, TokenSub, TokenSubAssign, TokenXor, TokenXorAssign,
};
use crate::dulang::parser::parser_error::ParserError;
use crate::dulang::parser::parser_error::ParserError::UnexpectedTokenError;
use crate::vm::instruction::OpCode::POP;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Result<Token, &'static str>,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            current_token: Ok(TokenHashTag {}),
            errors: Vec::new(),
        }
    }

    fn match_token(&mut self, expected_token: Token) -> bool {
        self.current_token = self.lexer.next_token();
        match &self.current_token {
            Ok(token) => {
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
        self.current_token = self.lexer.next_token();
        match &self.current_token {
            Ok(token) => {
                if *token != expected_token {
                    panic!("SyntaxError: expect token :{}", expected_token);
                }
            }
            _ => {
                panic!("SyntaxError: expect token :{}", expected_token);
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

    fn is_unary_op(&self, token: &Token) -> bool {
        return match token {
            TokenMul {} | TokenBand {} | TokenSub {} | TokenAdd {} => true,
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

    fn parse_type_spec(&mut self) -> Option<TypeSpec> {
        return None;
    }

    fn parse_expr_base(&mut self) -> Option<Expr> {
        return None;
    }

    fn parse_expr_unary(&mut self) -> Option<Expr> {
        let token = self.current_token.clone().unwrap();
        if self.is_unary_op(&token) {
            let token = self.lexer.next_token();
            self.current_token = token.clone();
            let right = self.parse_expr_unary().unwrap();
            return Some(UnaryExpr {
                op: token.unwrap(),
                operand: Box::new(self.parse_expr_unary().unwrap()),
            });
        } else {
            return self.parse_expr_base();
        }
    }

    fn parse_expr_mul(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_unary();
        let token = self.current_token.clone().unwrap();
        while self.is_unary_op(&token) {
            let token = self.lexer.next_token();
            self.current_token = token.clone();
            let right = self.parse_expr_unary().unwrap();
            expr = Some(BinaryExpr {
                op: token.unwrap(),
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn parse_expr_add(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_mul();
        let token = self.current_token.clone().unwrap();
        while self.is_add_op(&token) {
            let token = self.lexer.next_token();
            self.current_token = token.clone();
            let right = self.parse_expr_mul().unwrap();
            expr = Some(BinaryExpr {
                op: token.unwrap(),
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn parse_expr_cmp(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_add();
        let token = self.current_token.clone().unwrap();
        while self.is_cmp_op(&token) {
            let token = self.lexer.next_token();
            self.current_token = token.clone();
            let right = self.parse_expr_add().unwrap();
            expr = Some(BinaryExpr {
                op: token.unwrap(),
                left: Box::new(expr.unwrap()),
                right: Box::new(right),
            });
        }
        return expr;
    }

    fn parse_expr_and(&mut self) -> Option<Expr> {
        let mut expr = self.parse_expr_cmp();
        while self.match_token(TokenAdd {}) {
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
        while self.match_token(TokenOr {}) {
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
        if self.match_token(TokenQuestionMark {}) {
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

    fn parse_expr(&mut self) -> Option<Expr> {
        return self.parse_expr_ternary();
    }

    fn parse_name(&mut self) -> Option<String> {
        let token = self.lexer.next_token();
        self.current_token = token.clone();
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
        let token = self.lexer.next_token();
        self.current_token = token.clone();
        match token {
            Ok(token) => match token {
                TokenColon {} => {
                    let expr = self.parse_expr();
                    self.expect_token(TokenSemiColon {});
                    return Some(VarDecl {
                        name: name.unwrap(),
                        type_spec: None,
                        expr,
                    });
                }
                TokenAssign {} => {
                    let type_spec = self.parse_type_spec();
                    let mut expr = None;
                    if self.match_token(TokenAssign {}) {
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
    fn parse_decl_const(&self) -> Option<Decl> {
        return None;
    }
    fn parse_decl_func(&self) -> Option<Decl> {
        return None;
    }
    fn parse_decl_import(&self) -> Option<Decl> {
        return None;
    }

    fn parse_decl_opt(&mut self) -> Option<Decl> {
        match self.lexer.next_token() {
            Ok(keyword) => match keyword {
                TokenKeyword { keyword } => match keyword {
                    KeywordEnum { name } => {
                        return self.parse_decl_enum();
                    }
                    KeywordTypeDef { name } => {
                        return self.parse_decl_type_def();
                    }
                    KeywordStruct { name } => {
                        return self.parse_decl_struct();
                    }
                    KeywordVar { name } => {
                        return self.parse_decl_var();
                    }
                    KeywordConst { name } => {
                        return self.parse_decl_const();
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
            }
        }
        return None;
    }

    fn parse_decl(&mut self) -> Option<Decl> {
        let decl = self.parse_decl_opt();
        return decl;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn should_parse_var_decl() {
        let mut lexer = Lexer::new("var a = 1;");
        let mut parser = Parser::new(&mut lexer);
        let decl = parser.parse_decl();
        assert_eq!(
            decl.unwrap(),
            VarDecl {
                name: "a".to_string(),
                type_spec: None,
                expr: None,
            }
        );
    }
}
