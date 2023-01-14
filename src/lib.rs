pub mod splash;
pub mod prelude {
    pub use super::GameState;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Splash,
    Game,
}
