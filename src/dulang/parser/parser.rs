/*
 * Copyright (c) 2019. NeroYang
 */
use crate::dulang::lexer::lexer::Lexer;
use crate::dulang::parser::decl::Decl;
use crate::dulang::lexer::keyword::Keyword::{KeywordEnum, KeywordTypeDef, KeywordStruct, KeywordVar, KeywordConst, KeywordFunc, KeywordImport, KeywordGoto};
use crate::dulang::lexer::token::Token::{TokenName, TokenKeyword};
use crate::dulang::parser::parser_error::ParserError;
use crate::dulang::parser::parser_error::ParserError::UnexpectedTokenError;

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

