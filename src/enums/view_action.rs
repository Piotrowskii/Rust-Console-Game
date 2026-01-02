use crate::enums::player_type::PlayerType;

#[derive(Debug, Clone, Copy)]
pub enum ViewAction {
    GoToMain,
    GoToSettings,
    GoToGame(PlayerType),
    Quit,
    Nothing
}