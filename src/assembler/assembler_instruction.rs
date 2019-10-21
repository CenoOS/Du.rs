/*
 * Copyright (c) 2019. NeroYang
 */

use crate::assembler::token::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub token: Option<Token>,
    pub label: Option<Token>,
    pub directive: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,

}

impl AssemblerInstruction {
    pub fn new(token: Option<Token>, label: Option<Token>,
               directive: Option<Token>, operand1: Option<Token>,
               operand2: Option<Token>, operand3: Option<Token>) -> AssemblerInstruction {
        AssemblerInstruction {
            token,
            label,
            directive,
            operand1,
            operand2,
            operand3,
        }
    }

    pub fn is_label_declaration(&self) -> bool {
        match &self.label {
            Some(label) => {
                match label {
                    Token::LabelDeclaration{ name } => {
                        return true;
                    }
                    _ => { return false; }
                }
            }
            None => { return false; }
        }
    }


    pub fn is_label_usage(&self) -> bool {
        let mut label_usage = false;
        match &self.operand1 {
            Some(label) => {
                match label {
                    Token::LabelUsage{ name } => {
                        label_usage = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match &self.operand2 {
            Some(label) => {
                match label {
                    Token::LabelUsage{ name } => {
                        label_usage = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        match &self.operand3 {
            Some(label) => {
                match label {
                    Token::LabelUsage{ name } => {
                        label_usage = true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return label_usage;
    }

    pub fn is_opcode(&self) -> bool {
        return self.token.is_some();
    }

    pub fn get_label_declaration_name(&self) -> Option<String> {
        match &self.label {
            Some(label) => {
                match label {
                    Token::LabelDeclaration { name } => {
                        return Some(name.to_string());
                    }
                    _ => {
                        return None;
                    }
                }
            }
            _ => { return None; }
        }
    }


    pub fn get_label_usage_name(&self) -> Option<String> {
        match &self.label {
            Some(label) => {
                match label {
                    Token::LabelUsage { name } => {
                        return Some(name.to_string());
                    }
                    _ => {
                        return None;
                    }
                }
            }
            _ => { return None; }
        }
    }

    pub fn get_directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(label) => {
                match label {
                    Token::Directive { name } => {
                        return Some(name.to_string());
                    }
                    _ => {
                        return None;
                    }
                }
            }
            _ => { return None; }
        }
    }
    pub fn get_string_constant(&self) -> Option<String> {
        if self.get_directive_name().is_some() {
            match &self.operand1 {
                Some(token) => {
                    match token {
                        Token::IrString { name } => { return Some(name.to_string()); }
                        _ => { return None; }
                    }
                }
                None => { return None; }
            }
        } else {
            return None;
        }
    }

    pub fn is_directive(&self) -> bool {
        return self.directive.is_some();
    }

    pub fn has_operands(&self) -> bool {
        return self.operand1.is_some() || self.operand2.is_some() || self.operand3.is_some();
    }

    pub fn set_operand1(&mut self, token: Option<Token>) {
        self.operand1 = token;
    }

    pub fn set_operand2(&mut self, token: Option<Token>) {
        self.operand2 = token;
    }

    pub fn set_operand3(&mut self, token: Option<Token>) {
        self.operand3 = token;
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut results: Vec<u8> = Vec::new();
        match self.token {
            Some(Token::Op { opcode }) => match opcode {
                _ => { results.push(opcode as u8) }
            },
            _ => {
                println!("None opCode found in opCode field.");
                std::process::exit(0);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            match operand {
                Some(t) => {
                    match t {
                        Token::Register { reg_num } => {
                            results.push(*reg_num);
                        }
                        Token::IntegerOperand { value } => {
                            let converted = *value as u16;
                            let byte1 = converted;
                            let byte2 = converted >> 8;
                            results.push(byte2 as u8);
                            results.push(byte1 as u8);
                        }
                        _ => {
                            println!("Unexpected opcode in operand field.");
                            std::process::exit(0);
                        }
                    }
                }
                None => {}
            }
        }
        return results;
    }
}
