/*
 * Copyright (c) 2019. NeroYang
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::dolang::lexer::int::Int::{IntBin, IntHex, IntOct};
    use crate::dolang::lexer::keyword::Keyword::{
        KeywordBreak, KeywordCase, KeywordConst, KeywordContinue, KeywordDefault, KeywordDo,
        KeywordElse, KeywordEnum, KeywordFor, KeywordFunc, KeywordGoto, KeywordIf, KeywordImport,
        KeywordReturn, KeywordSizeOf, KeywordStruct, KeywordSwitch, KeywordTypeDef, KeywordTypeOf,
        KeywordVar, KeywordWhile,
    };
    use crate::dolang::lexer::lexer::Lexer;
    use crate::dolang::lexer::token::Token;
    use std::string::ToString;

    #[test]
    fn should_return_true_when_give_a_alpha() {
        for c in vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z',
        ] {
            assert_eq!(Lexer::is_alpha(&c), true);
        }
    }

    #[test]
    fn should_return_true_when_give_a_number() {
        for c in vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'] {
            assert_eq!(Lexer::is_digit(&c), true);
        }
    }

    #[test]
    fn should_return_true_when_give_a_number_or_alpha() {
        for c in vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
            'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
            'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ] {
            assert_eq!(Lexer::is_al_num(&c), true);
        }
    }

    #[test]
    fn should_return_token_char() {
        let mut lexer = Lexer::new(" 'a' 'b' 'E' '1' '0'");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: 'a' });

        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: 'b' });

        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: 'E' });

        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: '1' });

        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: '0' });
    }

    #[test]
    #[should_panic(expected = "SyntaxError: Char literal cannot contain newline")]
    fn should_throw_when_token_char_contains_new_line() {
        let mut lexer = Lexer::new(" '\n'");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenChar { value: '\n' });
    }

    #[test]
    fn should_return_token_str() {
        let mut lexer = Lexer::new("\"xxxx\" \"aaa\" \"111\" \"000000\" \"z\"");
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenStr {
                value: "xxxx".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenStr {
                value: "aaa".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenStr {
                value: "111".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenStr {
                value: "000000".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenStr {
                value: "z".to_string()
            }
        );
    }

    #[test]
    fn should_return_token_float() {
        let mut lexer = Lexer::new("1.324 .23 0.34 1.23e-1 1.22e+12 0.0 ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 1.324 });
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 0.23 });
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 0.34 });
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 1.23e-1 });
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 1.22e+12 });
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenFloat { value: 0.0 });
    }

    #[test]
    fn should_return_u8_when_give_a_hex_char() {
        let mut result = vec![];
        for c in vec![
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'A', 'b', 'B', 'c', 'C', 'd',
            'D', 'e', 'E', 'f', 'F',
        ] {
            result.push(Lexer::hex_char_to_digit(&c));
        }
        assert_eq!(
            result,
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15]
        );
    }

    #[test]
    fn should_return_token_int() {
        let mut lexer = Lexer::new("0xa 0b110 12345 0 321 ");
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntHex { value: 10 },
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntBin { value: 6 },
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 12345 },
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 0 },
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 321 },
            }
        );
    }

    #[test]
    fn should_return_token_keyword() {
        let mut lexer = Lexer::new(
            "typedef enum struct const let fn import goto \
             sizeof typeof \
             break continue return \
             if else while do for \
             switch case default ",
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordTypeDef {
                    name: "typedef".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordEnum {
                    name: "enum".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordStruct {
                    name: "struct".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordConst {
                    name: "const".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordVar {
                    name: "let".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordFunc {
                    name: "fn".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordImport {
                    name: "import".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordGoto {
                    name: "goto".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordSizeOf {
                    name: "sizeof".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordTypeOf {
                    name: "typeof".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordBreak {
                    name: "break".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordContinue {
                    name: "continue".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordReturn {
                    name: "return".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordIf {
                    name: "if".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordElse {
                    name: "else".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordWhile {
                    name: "while".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordDo {
                    name: "do".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordFor {
                    name: "for".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordSwitch {
                    name: "switch".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordCase {
                    name: "case".to_string()
                },
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordDefault {
                    name: "default".to_string()
                },
            }
        );
    }

    #[test]
    fn should_return_token_name() {
        let mut lexer = Lexer::new("name age _year address_detail phone_ email1 email2 ");
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "name".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "age".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "_year".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "address_detail".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "phone_".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "email1".to_string()
            }
        );

        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "email2".to_string()
            }
        );
    }

    #[test]
    fn should_return_token_less_than() {
        let mut lexer = Lexer::new("< ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLessThan {});
    }

    #[test]
    fn should_return_token_less_than_eq() {
        let mut lexer = Lexer::new("<= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLessThanEqual {});
    }

    #[test]
    fn should_return_token_left_shift() {
        let mut lexer = Lexer::new("<< ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLeftShift {});
    }

    #[test]
    fn should_return_token_left_shift_assign() {
        let mut lexer = Lexer::new("<<= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLeftShiftAssign {});
    }

    #[test]
    fn should_return_token_greater_than() {
        let mut lexer = Lexer::new("> ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenGreaterThan {});
    }

    #[test]
    fn should_return_token_greater_than_eq() {
        let mut lexer = Lexer::new(">= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenGreaterThanEqual {});
    }

    #[test]
    fn should_return_token_right_shift() {
        let mut lexer = Lexer::new(">> ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenRightShift {});
    }

    #[test]
    fn should_return_token_right_shift_assign() {
        let mut lexer = Lexer::new(">>= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenRightShiftAssign {});
    }

    #[test]
    fn should_return_token_not() {
        let mut lexer = Lexer::new("! ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenNot {});
    }

    #[test]
    fn should_return_token_not_equal() {
        let mut lexer = Lexer::new("!= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenNotEqual {});
    }

    #[test]
    fn should_return_token_colon() {
        let mut lexer = Lexer::new(": ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenColon {});
    }

    #[test]
    fn should_return_token_colon_assign() {
        let mut lexer = Lexer::new(":= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenColonAssign {});
    }

    #[test]
    fn should_return_token_xor() {
        let mut lexer = Lexer::new("^ ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenXor {});
    }

    #[test]
    fn should_return_token_xor_assign() {
        let mut lexer = Lexer::new("^= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenXorAssign {});
    }

    #[test]
    fn should_return_token_mul() {
        let mut lexer = Lexer::new("* ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenMul {});
    }

    #[test]
    fn should_return_token_mul_assign() {
        let mut lexer = Lexer::new("*= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenMulAssign {});
    }

    #[test]
    fn should_return_token_mod() {
        let mut lexer = Lexer::new("% ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenMod {});
    }

    #[test]
    fn should_return_token_mod_assign() {
        let mut lexer = Lexer::new("%= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenModAssign {});
    }

    #[test]
    fn should_return_token_add() {
        let mut lexer = Lexer::new("+ ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAdd {});
    }

    #[test]
    fn should_return_token_add_assign() {
        let mut lexer = Lexer::new("+= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAddAssign {});
    }

    #[test]
    fn should_return_token_inc_assign() {
        let mut lexer = Lexer::new("++ ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenInc {});
    }

    #[test]
    fn should_return_token_sub() {
        let mut lexer = Lexer::new("- ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenSub {});
    }

    #[test]
    fn should_return_token_sub_assign() {
        let mut lexer = Lexer::new("-= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenSubAssign {});
    }

    #[test]
    fn should_return_token_dec_assign() {
        let mut lexer = Lexer::new("-- ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenDec {});
    }

    #[test]
    fn should_return_token_bor() {
        let mut lexer = Lexer::new("| ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenBor {});
    }

    #[test]
    fn should_return_token_bor_assign() {
        let mut lexer = Lexer::new("|= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenOrAssign {});
    }

    #[test]
    fn should_return_token_or() {
        let mut lexer = Lexer::new("|| ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenOr {});
    }

    #[test]
    fn should_return_token_band() {
        let mut lexer = Lexer::new("& ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenBand {});
    }

    #[test]
    fn should_return_token_and_assign() {
        let mut lexer = Lexer::new("&= ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAndAssign {});
    }

    #[test]
    fn should_return_token_and() {
        let mut lexer = Lexer::new("&& ");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAnd {});
    }

    #[test]
    fn should_return_tokens_when_give_a_complex_str() {
        let mut lexer = Lexer::new("+ : := ++ -- += -= < > <= << >> >= <<= >>=");
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAdd {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenColon {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenColonAssign {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenInc {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenDec {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAddAssign {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenSubAssign {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLessThan {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenGreaterThan {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLessThanEqual {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLeftShift {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenRightShift {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenGreaterThanEqual {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLeftShiftAssign {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenRightShiftAssign {});
    }

    #[test]
    fn should_return_tokens_when_give_a_complex_str_2() {
        let mut lexer = Lexer::new("XY+(XY)_HELLO1,234+2147 ");
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "XY".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAdd {});
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenLeftBrackets {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "XY".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenRightBrackets {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "_HELLO1".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenComma {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 234 }
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAdd {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 2147 }
            }
        );
    }

    #[test]
    fn should_return_tokens_when_give_a_complex_str_3() {
        let mut lexer = Lexer::new("let x:int = 3 ");
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenKeyword {
                keyword: KeywordVar {
                    name: "let".to_string()
                }
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "x".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenColon {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenName {
                name: "int".to_string()
            }
        );
        let token_result = lexer.next_token();
        assert_eq!(token_result.unwrap(), Token::TokenAssign {});
        let token_result = lexer.next_token();
        assert_eq!(
            token_result.unwrap(),
            Token::TokenInt {
                value: IntOct { value: 3 }
            }
        );
    }
}
