mod views;
mod services;
mod traits;
mod enums;
mod helpers;

use std::rc::Rc;
use std::thread;
use std::time::Duration;
use ratatui::style::{Color, Style};
use crate::enums::player_type::PlayerType;
use crate::enums::view_action::ViewAction;
use crate::services::settings::Settings;
use crate::traits::view_model::ViewModel;
use crate::views::game_view::AiGameView;
use crate::views::main_view::MainView;

pub struct AppState{
    current_view: Box<dyn ViewModel>,
    settings: Rc<Settings>,
    running: bool,
}

impl AppState{
    pub fn new() -> AppState{
        let mut settings = Settings::new();
        settings.change_player_style(Style::new().fg(Color::Cyan));
        settings.change_opponent_style(Style::new().fg(Color::Yellow));

        AppState{
            current_view: Box::new(MainView::new()),
            running: true,
            settings: Rc::new(settings)
        }
    }

    pub fn handle_view_action(&mut self, action: ViewAction){
        match action{
            ViewAction::GoToMain => {self.go_to_main()}
            ViewAction::GoToGame(player_type) => {self.go_to_game_view(player_type)}
            ViewAction::GoToSettings => {self.go_to_settings()}
            ViewAction::Quit => {self.running = false}
            ViewAction::Nothing => {}
        }
    }

    fn go_to_main(&mut self){
        let view_model = MainView::new();
        self.current_view = Box::new(view_model);
    }

    fn go_to_settings(&mut self){

    }

    fn go_to_game_view(&mut self, player_type: PlayerType){
        let view_model = AiGameView::new(player_type, Rc::clone(&self.settings));
        self.current_view = Box::new(view_model);
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState::new();

    let mut terminal = ratatui::init();

    while state.running {
        terminal.draw(|frame| {
            let action = state.current_view.render(frame);
            state.handle_view_action(action);
        }).expect("Drawing terminal failed");
        thread::sleep(Duration::from_millis(17));
    }
    Ok(())
}