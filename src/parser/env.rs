use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::lexer::{LiteralTypes, Token};

#[derive(Debug, Clone)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, LiteralTypes>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, key: String, value: LiteralTypes) {
        self.values.insert(key, value);
    }

    pub fn get(&self, token: &Token) -> Option<LiteralTypes> {
        let key = &token.lexeme;
        if self.values.contains_key(key) {
            return self.values.get(key).map(Clone::clone);
        } else if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().borrow().get(token);
        }

        None
    }
    pub fn assign(&mut self, name: &Token, value: LiteralTypes) -> Result<(), ()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(());
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)?;
            return Ok(());
        }

        Err(())
    }
}
