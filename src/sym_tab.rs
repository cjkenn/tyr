use std::collections::HashMap;

/// SymbolTable is used to help determine program
/// addresses to jump to when executing jump
/// instructions.
pub struct SymbolTable {
    /// Hash table mapping a label name to an address in a program.
    table: HashMap<String, usize>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            table: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: String, val: usize) {
        self.table.insert(key, val);
    }

    pub fn get(&self, key: &String) -> Option<&usize> {
        self.table.get(key)
    }

    pub fn is_duplicate(&self, key: &String) -> bool {
        self.table.get(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_duplicate() {
        let mut sym_tab = SymbolTable::new();
        let key = "test".to_string();
        sym_tab.insert("test".to_string(), 5);

        let result = sym_tab.is_duplicate(&key);

        assert_eq!(result, true);
    }
}
