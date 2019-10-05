#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>
}

impl Symbol {
    pub fn new(name: String, offset: u32, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            offset,
            symbol_type,
        }
    }
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            symbols: Vec::new(),
        }
    }

    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    pub fn get_symbol_value(&mut self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s.to_string() {
                return Some(symbol.offset);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::assembler::symbol_table::{SymbolTable, Symbol};
    use crate::assembler::symbol_table::SymbolType::Label;

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
        let symbol_value = symbol_table.get_symbol_value("hello").unwrap();
        assert_eq!(symbol_value, 12);
    }
}
