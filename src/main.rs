mod views;
mod services;
mod traits;
mod enums;
mod helpers;

use crate::enums::view_action::ViewAction;
use crate::traits::view_model::ViewModel;
use crate::views::ai_game_view::AiGameView;
use crate::views::main_view::MainView;

pub struct AppState{
    current_view: Box<dyn ViewModel>,
    running: bool,
}

impl AppState{
    pub fn new() -> AppState{
        AppState{current_view: Box::new(MainView::new()), running: true}
    }

    pub fn handle_view_action(&mut self, action: ViewAction){
        match action{
            ViewAction::GoToMain => {self.go_to_main()}
            ViewAction::GoToAiGame => {self.go_to_ai_game()}
            ViewAction::GoToSettings => {self.go_to_settings()}
            ViewAction::Quit => {self.running = false}
            ViewAction::Nothing => {}
        }
    }

    pub fn go_to_main(&mut self){
        let view_model = MainView::new();
        self.current_view = Box::new(view_model);
    }

    pub fn go_to_settings(&mut self){

    }
    
    pub fn go_to_ai_game(&mut self){
        let view_model = AiGameView::new();
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
    }
    Ok(())
}