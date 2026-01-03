use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Style;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, List, ListState};
use crate::enums::player_type::PlayerType;
use crate::enums::view_action::ViewAction;
use crate::traits::view_model::ViewModel;
use crate::services::game_art as Art;

pub struct MainView{
    main_list: ListState,
    list_options: Vec<MenuOption>
}


#[derive(Debug)]
pub enum MenuOption{
    StartAiGame,
    StartLocalGame,
    Settings,
    Quit,
}

impl MenuOption{
    pub fn as_str(&self) -> &str{
        match self {
            MenuOption::StartAiGame => "Start Game with Ai",
            MenuOption::StartLocalGame => "Start Local Co-op game",
            MenuOption::Quit => "Quit",
            MenuOption::Settings => "Settings"
        }
    }
}

impl MainView{
    pub fn new() -> MainView{
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        MainView{
            list_options: vec![MenuOption::StartAiGame,MenuOption::StartLocalGame,MenuOption::Settings,MenuOption::Quit],
            main_list: list_state
        }
    }
    fn select_option(&mut self) -> ViewAction{
        let selected =self.get_selected_menu_option();
        match selected {
            Some(option) => {
                match option {
                    MenuOption::StartAiGame => ViewAction::GoToGame(PlayerType::Ai),
                    MenuOption::StartLocalGame => ViewAction::GoToGame(PlayerType::Human),
                    MenuOption::Settings => ViewAction::GoToSettings,
                    MenuOption::Quit => ViewAction::Quit,
                }
            }
            _ => ViewAction::Nothing
        }
    }

    fn get_selected_menu_option(&mut self) -> Option<&MenuOption> {
        let index = self.main_list.selected();
        if let Some(index) = index {
            self.list_options.get(index)
        }else{
            None
        }
    }
}
impl ViewModel for MainView{
    fn render_widgets(&mut self, frame: &mut Frame){

        let title = Art::tic_tac_toe();

        let main_layout_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(frame.area());

        let left_area_top = main_layout_rects[0]
            .centered(Constraint::Length(title.width() as u16),Constraint::Percentage(50));
        let left_area_bottom = main_layout_rects[1]
            .centered(Constraint::Percentage(75),Constraint::Percentage(75));

        frame.render_widget(title, left_area_top);

        let list_items = self.list_options.iter().map(|item|{item.as_str()}).collect::<Vec<&str>>();
        let list = List::new(list_items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, left_area_bottom, &mut self.main_list);

        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded), frame.area());

    }
    fn handle_inputs(&mut self, key: KeyEvent) -> ViewAction {
        match key.code {
            KeyCode::Esc => return ViewAction::Quit,
            KeyCode::Up => self.main_list.select_previous(),
            KeyCode::Down => self.main_list.select_next(),
            KeyCode::Enter => return self.select_option(),
            _ => {}
        }
        ViewAction::Nothing
    }

    fn additional_actions(&mut self) {
        
    }
}



