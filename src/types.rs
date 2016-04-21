use env::Env;

#[derive(Clone)]
pub enum MalType {
    True,
    False,
    Nil,
    Int(i32),
    Symbol(String),
    String(String),
    List(Vec<MalType>),
    Func(fn(Vec<MalType>) -> MalType),
    MalFunc(Box<MalType>, Box<MalType>, Env),
    Error(String),
}
