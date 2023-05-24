pub struct Player {
    name: String,
    symbol: char,
}
impl Player {
    pub(crate) fn default() -> Player {
        Player {
            name: "Player".to_string(),
            symbol: 'X',
        }
    }
}
impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            name: self.name.clone(),
            symbol: self.symbol,
        }
    }
}
