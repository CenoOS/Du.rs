/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::lexer::lexer::Lexer;
use crate::dulang::parser::decl::Decl;
use crate::dulang::lexer::keyword::Keyword::{KeywordEnum, KeywordTypeDef, KeywordStruct, KeywordVar, KeywordConst, KeywordFunc, KeywordImport, KeywordGoto};
use crate::dulang::lexer::token::Token::{TokenName, TokenKeyword, TokenColon, TokenAssign, TokenSemiColon};
use crate::dulang::parser::parser_error::ParserError;
use crate::dulang::parser::parser_error::ParserError::UnexpectedTokenError;
use crate::dulang::lexer::token::Token;
use crate::dulang::parser::decl::Decl::VarDecl;
use crate::dulang::parser::expr::Expr;
use std::process::exit;
use crate::dulang::parser::type_spec::TypeSpec;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    errors: Vec<ParserError>,
}


impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            errors: Vec::new(),
        }
    }

    fn match_token(&mut self, expected_token: Token) -> bool {
        let token = self.lexer.next_token();
        match token {
            Ok(token) => {
                if token == expected_token {
                    return true;
                }
                return false;
            }
            _ => { return false; }
        }
    }

    fn expect_token(&mut self, expected_token: Token) {
        let token = self.lexer.next_token();
        match token {
            Ok(token) => {
                if token != expected_token {
                    panic!("SyntaxError: expect token :{}", expected_token);
                }
            }
            _ => { panic!("SyntaxError: expect token :{}", expected_token); }
        }
    }

    fn parse_type_spec(&mut self) -> Option<TypeSpec> {
        return None;
    }

    fn parse_expr(&mut self) -> Option<Expr> {
        return None;
    }

    fn parse_name(&mut self) -> Option<String> {
        let name = self.lexer.next_token();
        match name {
            Ok(TokenName { name }) => { return Some(name); }
            _ => {
                let error = UnexpectedTokenError { token: name.unwrap(), line: 0 };
                self.errors.push(error);
                return None;
            }
        }
    }

    fn parse_decl_enum(&self) -> Option<Decl> { return None; }
    fn parse_decl_type_def(&self) -> Option<Decl> { return None; }
    fn parse_decl_struct(&self) -> Option<Decl> { return None; }
    fn parse_decl_var(&mut self) -> Option<Decl> {
        let name = self.parse_name();
        let token = self.lexer.next_token();
        match token {
            Ok(token) => {
                match token {
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
                }
            }
            _ => { self.errors.push(UnexpectedTokenError { token: token.unwrap(), line: 0 }); }
        }
        return None;
    }
    fn parse_decl_const(&self) -> Option<Decl> { return None; }
    fn parse_decl_func(&self) -> Option<Decl> { return None; }
    fn parse_decl_import(&self) -> Option<Decl> { return None; }

    fn parse_decl_opt(&mut self) -> Option<Decl> {
        match self.lexer.next_token() {
            Ok(keyword) => {
                match keyword {
                    TokenKeyword { keyword } => {
                        match keyword {
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
                        }
                    }
                    Token => {}
                }
            }
            _ => { return None; }
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
        parser.parse_decl();
    }
}

