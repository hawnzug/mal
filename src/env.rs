use std::collections::HashMap;
use types::MalType;

pub struct Env {
    data: HashMap<String, MalType>,
    outer: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            data: HashMap::new(),
            outer: None,
        }
    }
    pub fn set(&mut self, symbol: String, m: MalType) {
        self.data.insert(symbol, m);
    }

    pub fn find(&self, symbol: &str) -> Option<MalType> {
        if let Some(m) = self.data.get(symbol) {
            return Some(m.clone());
        } else {
            match self.outer {
                Some(ref e) => e.find(symbol),
                None => None,
            }
        }
    }
}
