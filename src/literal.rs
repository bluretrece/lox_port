use std::fmt;
#[derive(Debug,PartialEq,Clone, PartialOrd, Eq)]
pub enum Literal {
    String(String),
    Number(i32),
    //FloatNumber(f32),
    None,
    Boolean(bool)
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::None => write!(f, "nil"),
            Self::Number(n) => {
                let mut x = n.to_string();
                write!(f, "{}", x)
            },
            Self::Boolean(x) => {
                write!(f, "{}", x)
            }
        }
    }
}
