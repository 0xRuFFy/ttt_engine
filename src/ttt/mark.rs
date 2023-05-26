use std::fmt::Display;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mark {
    X,
    O,
}

impl Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = match self {
            Mark::X => "X",
            Mark::O => "O",
        };
        write!(f, "{}", mark)
    }
}