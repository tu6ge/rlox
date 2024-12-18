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
    pub fn assign(&mut self, name: &Token, value: LiteralTypes) -> Result<(), ()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        }

        Err(())
    }
}
