#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Promotion,
    Check,
    GameOver
}