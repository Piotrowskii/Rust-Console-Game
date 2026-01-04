use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin};
use ratatui::style::{Color, Style, Styled};
use ratatui::widgets::{Block, BorderType, Borders, Clear, List, ListState};
use crate::enums::player::Player;
use crate::enums::view_action::ViewAction;
use crate::traits::view_model::ViewModel;
use crate::services::game_art as Art;

pub struct SettingsView{
    main_list: ListState,
    color_list: ListState,
    list_options: Vec<MenuOption>,
    color_options: Vec<ColorOption>,
    new_color_player: Player,
    show_modal: bool,
}


#[derive(Debug)]
pub enum MenuOption{
    ChangeOpponentColor,
    ChangePlayerColor,
    Quit
}

impl MenuOption{
    pub fn as_str(&self) -> &str{
        match self {
            MenuOption::ChangeOpponentColor => "Change opponent color",
            MenuOption::ChangePlayerColor => "Change player color",
            MenuOption::Quit => "Go back"
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub enum ColorOption{
    Green,
    Blue,
    Magenta,
    Yellow,
    Red,
    Miku,
    MorningSun,
    Pink,
    Orange
}

impl ColorOption{
    pub fn as_color(&self) -> Color{
        match self {
            ColorOption::Green => {Color::Green}
            ColorOption::Blue => {Color::Blue}
            ColorOption::Magenta => {Color::Magenta}
            ColorOption::Yellow => {Color::Yellow},
            ColorOption::Red => {Color::Red},
            ColorOption::Miku => {Color::Rgb(134,206,203)},
            ColorOption::MorningSun => {Color::Rgb(255,255,26)}
            ColorOption::Pink => {Color::Rgb(255,51,255)}
            ColorOption::Orange => {Color::Rgb(255,102,0)}
        }
    }

    pub fn get_all() -> [ColorOption; 9]{
        [
            ColorOption::Green,
            ColorOption::Blue,
            ColorOption::Magenta,
            ColorOption::Yellow,
            ColorOption::Red,
            ColorOption::Miku,
            ColorOption::MorningSun,
            ColorOption::Pink,
            ColorOption::Orange
        ]
    }
}

impl SettingsView{
    pub fn new() -> SettingsView{
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut list_state2 = ListState::default();
        list_state2.select(Some(0));

        SettingsView{
            list_options: vec![MenuOption::ChangePlayerColor,MenuOption::ChangeOpponentColor,MenuOption::Quit],
            color_options: ColorOption::get_all().to_vec(),
            main_list: list_state,
            show_modal: false,
            color_list: list_state2,
            new_color_player: Player::Player,
        }
    }
    fn select_menu_option(&mut self) -> Option<ViewAction>{
        let selected =self.get_selected_menu_option();
        match selected {
            Some(option) => {
                match option {
                    MenuOption::ChangeOpponentColor => {
                        self.change_new_player_color(Player::Opponent);
                        self.toggle_modal()
                    }
                    MenuOption::ChangePlayerColor => {
                        self.change_new_player_color(Player::Player);
                        self.toggle_modal()
                    }
                    MenuOption::Quit => {return Some(ViewAction::GoToMain)}
                }
            }
            _ => {}
        }
        None
    }

    fn get_selected_menu_option(&mut self) -> Option<&MenuOption> {
        let index = self.main_list.selected();
        if let Some(index) = index {
            self.list_options.get(index)
        }else{
            None
        }
    }

    fn get_selected_color_option(&mut self) -> Option<&ColorOption> {
        let index = self.color_list.selected();
        if let Some(index) = index {
            self.color_options.get(index)
        }
        else{
            None
        }
    }

    fn select_color_option(&mut self) -> Option<ViewAction> {
        if let Some(color_option) = self.get_selected_color_option(){
            return Some(ViewAction::ChangeColor((color_option.as_color(),self.new_color_player)));
        }
        None
    }

    fn change_new_player_color(&mut self,player: Player){
        self.new_color_player = player;
    }

    fn toggle_modal(&mut self){
        self.show_modal = !self.show_modal;
    }

    fn draw_modal(&mut self, frame: &mut Frame) {
        if self.show_modal {
            let popup_area = frame.area().centered(Constraint::Percentage(50),Constraint::Percentage(50));
            let popup_area_rects = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80),Constraint::Percentage(20)])
                .split(popup_area.inner(Margin::new(1,1)));

            let popup_top = popup_area_rects[0];
            let popup_bottom = popup_area_rects[1];

            //rendering border
            let block = Block::bordered().title("Select color").title_alignment(Alignment::Center);
            frame.render_widget(Clear, popup_area); //Clear area
            frame.render_widget(block, popup_area);

            //rendering color list
            let list_items = self.color_options.iter().map(|color| format!("{:?}",color)).collect::<Vec<String>>();
            let list = List::new(list_items)
                .highlight_style(Style::new().reversed())
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true);

            frame.render_stateful_widget(list, popup_top, &mut self.color_list);

            //rendering color preview
            if let Some(color) = self.get_selected_color_option(){
                let color_block = Block::new().style(Style::new().bg(color.as_color()));
                frame.render_widget(color_block, popup_bottom);
            }
        }
    }

    fn handle_input_menu(&mut self, key: KeyEvent) -> ViewAction{
        match key.code {
            KeyCode::Esc => return ViewAction::GoToMain,
            KeyCode::Up => self.main_list.select_previous(),
            KeyCode::Down => self.main_list.select_next(),
            KeyCode::Enter =>  match self.select_menu_option() {
                None => {}
                Some(action) => {return action;}
            },
            _ => {}
        }
        ViewAction::Nothing
    }

    fn handle_input_modal(&mut self, key: KeyEvent)-> ViewAction{
        match key.code {
            KeyCode::Esc => self.toggle_modal(),
            KeyCode::Up => self.color_list.select_previous(),
            KeyCode::Down => self.color_list.select_next(),
            KeyCode::Enter => {
                self.toggle_modal();
                if let Some(action) = self.select_color_option(){
                    self.color_list.select(Some(0));
                    return action;
                }
            }
            _ => {}
        }

        ViewAction::Nothing
    }
}
impl ViewModel for SettingsView{
    fn render_widgets(&mut self, frame: &mut Frame){

        let title = Art::settings();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(100),
            ])
            .split(frame.area());

        let left_area = main_layout[0];
        let left_area_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(left_area);

        let left_area_top = left_area_rects[0]
            .centered(Constraint::Length(title.width() as u16),Constraint::Percentage(50));
        let left_area_bottom = left_area_rects[1]
            .centered(Constraint::Percentage(75),Constraint::Percentage(75));

        frame.render_widget(title, left_area_top);

        let list_items = self.list_options.iter().map(|item|{item.as_str()}).collect::<Vec<&str>>();
        let list = List::new(list_items)
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true);

        frame.render_stateful_widget(list, left_area_bottom, &mut self.main_list);

        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded), left_area);

        //draw modal
        self.draw_modal(frame)

    }
    fn handle_inputs(&mut self, key: KeyEvent) -> ViewAction {
        if(self.show_modal){
            return self.handle_input_modal(key)
        }else{
            return self.handle_input_menu(key);
        }

        ViewAction::Nothing
    }

    fn additional_actions(&mut self) {

    }
}



