use std::collections::HashMap;
use types::MalType;

#[derive(Clone)]
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

    pub fn extend(self, formals: MalType, args: Vec<MalType>) -> Option<Env> {
        let mut newenv = Env {
            data: HashMap::new(),
            outer: Some(Box::new(self)),
        };
        let len = args.len();
        if let MalType::List(form) = formals {
            for i in 0..len {
                if let MalType::Symbol(ref s) = form[i] {
                    newenv.set(s.clone(), args[i].clone());
                } else {
                    return None;
                }
            }
            Some(newenv)
        } else {
            None
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
