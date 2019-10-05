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
