use ratatui::style::Color;
use crate::enums::player::Player;
use crate::enums::player_type::PlayerType;

#[derive(Debug, Clone, Copy)]
pub enum ViewAction {
    GoToMain,
    GoToSettings,
    GoToGame(PlayerType),
    ChangeColor((Color, Player)),
    Quit,
    Nothing
}