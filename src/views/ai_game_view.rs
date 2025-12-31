use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::crossterm::event::KeyCode::F;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use crate::enums::field::FieldMark;
use crate::enums::player::Player;
use crate::enums::view_action::ViewAction;
use crate::services::game::Game;
use crate::traits::view_model::ViewModel;

pub struct AiGameView{
    game: Game,
    field_selection: u8,
}

impl AiGameView{

    pub fn new() -> AiGameView{
        AiGameView{
            game: Game::new(),
            field_selection: 7,
        }
    }

    fn get_board_tiles(centered_board: Rect) -> Vec<Rect>{
        let mut fields: Vec<Rect> = Vec::new();

        let vertical_fields = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1)]
            )
            .split(centered_board);

        for &vertical_field in vertical_fields.iter(){
            let horizontal_fields = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1)]
                )
                .split(vertical_field);
            for &horizontal_field in horizontal_fields.iter(){
                fields.push(horizontal_field);
            }
        }

        fields
    }

    fn get_your_turn_text<'a>() -> Text<'a>{
        let title: Vec<Line> = vec![
            Line::from("__   __                 _                    "),
            Line::from("\\ \\ / /__  _   _ _ __  | |_ _   _ _ __ _ __  "),
            Line::from(" \\ V / _ \\| | | | '__| | __| | | | '__| '_ \\ "),
            Line::from("  | | (_) | |_| | |    | |_| |_| | |  | | | |"),
            Line::from("  |_|\\___/ \\__,_|_|     \\__|\\__,_|_|  |_| |_|"),
        ];
        Text::from(title).style(Self::get_player_style())
    }

    fn get_enemy_turn_text<'a>() -> Text<'a>{
        let title: Vec<Line> = vec![
            Line::from(" _____                              _                    "),
            Line::from("| ____|_ __   ___ _ __ ___  _   _  | |_ _   _ _ __ _ __  "),
            Line::from("|  _| | '_ \\ / _ \\ '_ ` _ \\| | | | | __| | | | '__| '_ \\ "),
            Line::from("| |___| | | |  __/ | | | | | |_| | | |_| |_| | |  | | | |"),
            Line::from("|_____|_| |_|\\___|_| |_| |_|\\__, |  \\__|\\__,_|_|  |_| |_|"),
            Line::from("                            |___/                        ")
        ];
        Text::from(title).style(Self::get_opponent_style())
    }

    fn get_win_text<'a>() -> Text<'a>{
        let title: Vec<Line> = vec![
            Line::from("__   __                                "),
            Line::from("\\ \\ / /__  _   _  __      _____  _ __  "),
            Line::from(" \\ V / _ \\| | | | \\ \\ /\\ / / _ \\| '_ \\ "),
            Line::from("  | | (_) | |_| |  \\ V  V / (_) | | | |"),
            Line::from("  |_|\\___/ \\__,_|   \\_/\\_/ \\___/|_| |_|"),
        ];
        Text::from(title).style(Style::new().green())
    }

    fn get_lost_text<'a>() -> Text<'a>{
        let title: Vec<Line> = vec![
            Line::from("__   __            _           _   "),
            Line::from("\\ \\ / /__  _   _  | | ___  ___| |_ "),
            Line::from(" \\ V / _ \\| | | | | |/ _ \\/ __| __|"),
            Line::from("  | | (_) | |_| | | | (_) \\__ \\ |_ "),
            Line::from("  |_|\\___/ \\__,_| |_|\\___/|___/\\__|"),
        ];
        Text::from(title).style(Style::new().red())
    }

    fn get_draw_text<'a>() -> Text<'a>{
        let title: Vec<Line> = vec![
            Line::from(" ____  ____      ___        __"),
            Line::from("|  _ \\|  _ \\    / \\ \\      / /"),
            Line::from("| | | | |_) |  / _ \\ \\ /\\ / / "),
            Line::from("| |_| |  _ <  / ___ \\ V  V /"),
            Line::from("|____/|_| \\_\\/_/   \\_\\_/\\_/   ")
        ];
        Text::from(title).style(Style::new().gray())
    }

    fn get_x<'a>() -> Text<'a>{
        let x: Vec<Line> = vec![
            Line::from("__  __"),
            Line::from("\\ \\/ /"),
            Line::from(" >  < "),
            Line::from("/_/\\_\\"),
        ];

        Text::from(x)
    }

    fn get_o<'a>() -> Text<'a>{
        let o: Vec<Line> = vec![
            Line::from("  ___  "),
            Line::from(" / _ \\ "),
            Line::from("| (_) |"),
            Line::from(" \\___/ "),
        ];
        Text::from(o)
    }

    fn get_field_mark_art(&self, field_mark: &FieldMark) -> Text{
        let mark = match field_mark {
            FieldMark::X => {Self::get_x()},
            FieldMark::O => {Self::get_o()}
            FieldMark::Empty => {Text::from("")}
        };
        let style = if field_mark == &self.game.player_mark {Self::get_player_style()} else {Self::get_opponent_style()};
        mark.style(style)
    }

    fn get_player_style() -> Style{
        Style::new().blue()
    }

    fn get_opponent_style() -> Style{
        Style::new().red()
    }

    fn get_top_text<'a>(&self) -> Text<'a>{
        match self.game.winner {
            None => {
                match self.game.current_player {
                    Player::Player => {Self::get_your_turn_text()}
                    Player::Opponent => {Self::get_enemy_turn_text()}
                }
            }
            Some(winner_mark) => {
                if(winner_mark == self.game.player_mark){
                    Self::get_win_text()
                }else if(winner_mark == self.game.opponent_mark){
                    Self::get_lost_text()
                }
                else{
                    Self::get_draw_text()
                }
            }
        }
    }

    fn draw_board_tiles(&self, frame: &mut Frame, board_tiles: &[Rect]){
        assert_eq!(board_tiles.len(), self.game.board.len());

        for (i, mark) in self.game.board.iter().enumerate(){
            let field = board_tiles[i];
            let mut block = Block::new().borders(Borders::ALL).border_type(BorderType::Rounded);

            if(i == self.field_selection as usize && matches!(self.game.current_player, Player::Player)){
                block = block.style(Self::get_player_style());
            }

            let mark_art = self.get_field_mark_art(mark);

            frame.render_widget(Paragraph::new(mark_art).block(block),field);

        }

    }

    fn draw_error_text(&self,text: String){

    }

    fn move_selection_up(&mut self){
        if(self.field_selection > 2){self.field_selection -= 3;}
    }

    fn move_selection_down(&mut self){
        if(self.field_selection < 6){self.field_selection += 3;}
    }

    fn move_selection_left(&mut self){
        if(self.field_selection > 0){self.field_selection -= 1;}
    }

    fn move_selection_right(&mut self){
        if(self.field_selection < 8){self.field_selection += 1;}
    }

    fn player_make_move(&mut self){
        if(self.field_selection >= 0 && self.field_selection < 9){
            if let Err(message) = self.game.make_move(self.field_selection){
                self.draw_error_text(message);
            }
        }
    }

    fn opponent_make_move(&mut self){
        if(self.game.winner.is_none() && self.game.current_player == Player::Opponent){
            let ai_move = self.game.get_ai_move(Player::Opponent);
            match ai_move {
                None => {}
                Some(index) => {
                    match self.game.make_move(index){
                        Ok(_) => {}
                        Err(message) => {self.draw_error_text(message);}
                    }
                }
            }
        }
    }

    fn handle_input_your_turn(&mut self, key: KeyEvent) -> ViewAction{
        match key.code {
            KeyCode::Esc => return ViewAction::GoToMain,
            KeyCode::Up => {self.move_selection_up();}
            KeyCode::Down => {self.move_selection_down();}
            KeyCode::Left => {self.move_selection_left();}
            KeyCode::Right => {self.move_selection_right();}
            KeyCode::Enter => {self.player_make_move();}
            _ => {}
        }
        ViewAction::Nothing
    }

    fn handle_input_enemy_turn(&mut self, key: KeyEvent) -> ViewAction{
        match key.code {
            KeyCode::Esc => return ViewAction::GoToMain,
            _ => {}
        }

        ViewAction::Nothing
    }

    fn handle_input_end(&mut self, key: KeyEvent) -> ViewAction{
        match key.code {
            KeyCode::Esc => return ViewAction::GoToMain,
            _ => {}
        }

        ViewAction::Nothing
    }

}
impl ViewModel for AiGameView{
    fn render_widgets(&mut self, frame: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(75),
                Constraint::Percentage(25),
            ])
            .split(frame.area());

        let left_area = main_layout[0];
        let right_area = main_layout[1];
        let left_area_rects = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(18),
                Constraint::Length(2),
            ])
            .split(left_area);

        let board_area = left_area_rects[1]
            .centered_horizontally(Constraint::Length(33));

        let right_area_center = right_area
            .centered(Constraint::Percentage(50),Constraint::Percentage(50));

        //Drawing left/right separation
        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded), left_area);
        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded), right_area);

        //Drawing text above board
        let text = self.get_top_text();
        let above_board_area = left_area_rects[0]
            .inner(Margin::new(0,2))
            .centered_horizontally(Constraint::Length(text.width() as u16));
        frame.render_widget(text, above_board_area);

        //Drawing board tiles
        let board_tiles = AiGameView::get_board_tiles(board_area);
        self.draw_board_tiles(frame, &board_tiles);

    }
    fn handle_inputs(&mut self, key: KeyEvent) -> ViewAction {
        if(self.game.winner.is_some()){
            return self.handle_input_end(key);
        }else if(self.game.current_player == Player::Player){
            return self.handle_input_your_turn(key);
        }else {
            return self.handle_input_enemy_turn(key);
        }

        ViewAction::Nothing
    }

    fn additional_actions(&mut self) {
        self.opponent_make_move();
    }
}



