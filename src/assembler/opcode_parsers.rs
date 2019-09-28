use std::iter::Peekable;
use std::str::SplitWhitespace;
use crate::assembler::Token;
use crate::assembler::Token::Op;
use crate::instruction::OpCode::{LOAD, ADD, HLT, SUB, MUL, DIV, JMP, JMP_F, JMP_B, EQ, JEQ};
use std::string::ParseError;


pub struct OpCodeParser<'a> {
    tokens: Peekable<SplitWhitespace<'a>>,
}

impl<'a> OpCodeParser<'a> {
    pub fn new(str: &str) -> OpCodeParser {
        OpCodeParser {
            tokens: str.split_whitespace().peekable()
        }
    }

    pub fn parse_instruction(&mut self) -> Result<Token, &'static str> {
        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "HLT".to_string()) {
            return Ok(Op { opcode: HLT });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "LOAD".to_string()) {
            return Ok(Op { opcode: LOAD });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "ADD".to_string()) {
            return Ok(Op { opcode: ADD });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "SUB".to_string()) {
            return Ok(Op { opcode: SUB });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "MUL".to_string()) {
            return Ok(Op { opcode: MUL });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "DIV".to_string()) {
            return Ok(Op { opcode: DIV });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP".to_string()) {
            return Ok(Op { opcode: JMP });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP_F".to_string()) {
            return Ok(Op { opcode: JMP_F });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JMP_B".to_string()) {
            return Ok(Op { opcode: JMP_B });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "EQ".to_string()) {
            return Ok(Op { opcode: EQ });
        }

        if self.tokens.peek().map_or(false, |word| (*word).to_uppercase() == "JEQ".to_string()) {
            return Ok(Op { opcode: JEQ });
        }

        Err("Unexpected Assembly Code.")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_load_when_give_hlt() {
        let mut tokenParser = OpCodeParser::new("hlt");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: HLT });
    }

    #[test]
    fn should_return_load_when_give_load() {
        let mut tokenParser = OpCodeParser::new("load $1 #300");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: LOAD });
    }

    #[test]
    fn should_return_load_when_give_add() {
        let mut tokenParser = OpCodeParser::new("add $0 $1 $0");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: ADD });
    }

    #[test]
    fn should_return_load_when_give_sub() {
        let mut tokenParser = OpCodeParser::new("sub $0 $1 $0");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: SUB });
    }

    #[test]
    fn should_return_load_when_give_mul() {
        let mut tokenParser = OpCodeParser::new("mul $0 $1 $0");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: MUL });
    }

    #[test]
    fn should_return_load_when_give_div() {
        let mut tokenParser = OpCodeParser::new("div $0 $1 $0");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: DIV });
    }

    #[test]
    fn should_return_load_when_give_jmp() {
        let mut tokenParser = OpCodeParser::new("jmp $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: JMP });
    }

    #[test]
    fn should_return_load_when_give_jmp_f() {
        let mut tokenParser = OpCodeParser::new("jmp_f $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: JMP_F });
    }

    #[test]
    fn should_return_load_when_give_jmp_b() {
        let mut tokenParser = OpCodeParser::new("jmp_b $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: JMP_B });
    }

    #[test]
    fn should_return_load_when_give_eq() {
        let mut tokenParser = OpCodeParser::new("eq $1 $2");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: EQ });
    }

    #[test]
    fn should_return_load_when_give_jeq() {
        let mut tokenParser = OpCodeParser::new("jeq $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.unwrap(), Op { opcode: JEQ });
    }


    #[test]
    fn should_return_error_when_give_unexpected_token() {
        let mut tokenParser = OpCodeParser::new("xxx $1");
        let token = tokenParser.parse_instruction();
        assert_eq!(token.is_err(), true);
    }


}
