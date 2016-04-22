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

    pub fn extend(&mut self, env: &Env) {
        if let Some(ref mut b) = self.outer {
            (*b).extend(&env);
        } else {
            self.outer = Some(Box::new(env.clone()));
        }
    }

    pub fn multibind(self, formals: MalType, args: Vec<MalType>) -> Result<Env, MalType> {
        let mut newenv = Env {
            data: HashMap::new(),
            outer: Some(Box::new(self)),
        };
        let len = args.len();
        if let MalType::List(form) = formals {
            if form.len() == len {
                for i in 0..len {
                    if let MalType::Symbol(ref s) = form[i] {
                        newenv.set(s.clone(), args[i].clone());
                    } else {
                        return Err(MalType::Error("lambda: expect symbols in first parameter"
                                                      .to_string()));
                    }
                }
                Ok(newenv)
            } else {
                Err(MalType::Error("wrong argument number".to_string()))
            }
        } else {
            Err(MalType::Error("lambda: first parameter should be a list".to_string()))
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
