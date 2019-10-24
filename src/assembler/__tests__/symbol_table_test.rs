/*
 * Copyright (c) 2019. NeroYang
 */

#[cfg(test)]
mod tests {
    use crate::assembler::symbol_table::SymbolType::Label;
    use crate::assembler::symbol_table::{Symbol, SymbolTable};

    #[test]
    fn should_save_symbol_to_symbol_table_when_give_a_symbol() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_symbol(Symbol::new("hello".parse().unwrap(), 12, Label));
        assert_eq!(symbol_table.symbols.len(), 1);
    }

    #[test]
    fn should_get_symbol_from_symbol_table_when_give_a_symbol_name() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_symbol(Symbol::new("hello".parse().unwrap(), 12, Label));
        let symbol_value = symbol_table.get_symbol_offset("hello").unwrap();
        assert_eq!(symbol_value, 12);
    }

    #[test]
    fn should_set_symbol_offset_when_give_a_symbol_name_and_offset() {
        let mut symbol_table = SymbolTable::new();
        symbol_table.add_symbol(Symbol::new("hello".parse().unwrap(), 12, Label));
        let symbol_value = symbol_table.get_symbol_offset("hello").unwrap();
        assert_eq!(symbol_value, 12);
        symbol_table.set_symbol_offset("hello", 15);
        let symbol_value_new = symbol_table.get_symbol_offset("hello").unwrap();
        assert_eq!(symbol_value_new, 15);
    }
}
