#[derive(Debug, PartialEq)]
pub enum SymbolType {
    Label,
}

#[derive(Debug, PartialEq)]
pub struct Symbol {
    name: String,
    offset: u32,
    symbol_type: SymbolType,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub(crate) symbols: Vec<Symbol>,
}

impl Symbol {
    pub fn default(name: String, symbol_type: SymbolType) -> Symbol {
        Symbol {
            name,
            offset: 0,
            symbol_type,
        }
    }
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

    pub fn get_symbol(&mut self, s: &str) -> Option<&Symbol> {
        for symbol in &self.symbols {
            if symbol.name == s.to_string() {
                return Some(symbol);
            }
        }
        None
    }

    pub fn get_symbol_offset(&mut self, s: &str) -> Option<u32> {
        for symbol in &self.symbols {
            if symbol.name == s.to_string() {
                return Some(symbol.offset);
            }
        }
        None
    }

    pub fn set_symbol_offset(&mut self, s: &str, offset: u32) {
        for symbol in &mut self.symbols {
            if symbol.name == s.to_string() {
                symbol.offset = offset;
            }
        }
    }
}
