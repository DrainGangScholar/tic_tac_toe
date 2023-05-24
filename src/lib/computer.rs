pub struct Computer {
    name: String,
    symbol: char,
}
impl Computer {
    pub(crate) fn default() -> Computer {
        Computer {
            name: "Computer".to_string(),
            symbol: 'O',
        }
    }
}

impl Clone for Computer {
    fn clone(&self) -> Self {
        Computer {
            name: self.name.clone(),
            symbol: self.symbol,
        }
    }
}
