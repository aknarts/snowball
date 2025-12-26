use fin_engine::GameState;
use std::rc::Rc;
use yew::prelude::*;

/// Global application state
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AppState {
    /// Showing initialization screen (new game)
    #[default]
    Initialization,
    /// Game is active with a loaded state
    Playing { game_state: Rc<GameState> },
}

/// Actions that can modify the app state
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum AppAction {
    /// Start a new game with the given state
    StartGame(GameState),
    /// Update the game state
    UpdateGameState(GameState),
    /// Return to initialization (new game)
    ResetToInitialization,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppAction::StartGame(game_state) => Rc::new(AppState::Playing {
                game_state: Rc::new(game_state),
            }),
            AppAction::UpdateGameState(game_state) => Rc::new(AppState::Playing {
                game_state: Rc::new(game_state),
            }),
            AppAction::ResetToInitialization => Rc::new(AppState::Initialization),
        }
    }
}
