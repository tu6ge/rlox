use std::collections::HashMap;

use crate::lexer::{LiteralTypes, Token};

pub struct Environment {
    values: HashMap<String, LiteralTypes>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, key: String, value: LiteralTypes) {
        self.values.insert(key, value);
    }

    pub fn get(&self, token: &Token) -> Option<LiteralTypes> {
        let key = &token.lexeme;
        self.values.get(key).map(Clone::clone)
    }
}
