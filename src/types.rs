pub enum MalType {
    True,
    False,
    Nil,
    Int(i32),
    Symbol(String),
    List(Vec<MalType>),
    Error(String),
}
