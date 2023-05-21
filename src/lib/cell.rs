use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    X,
    O,
}
impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)
    }
}
