#[derive(Debug, PartialEq, PartialOrd, Eq)]
pub enum Literal {
    String(String),
    Number(i32),
    None,
}
