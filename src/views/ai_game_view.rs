use std::thread;
use std::time::Duration;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Rect};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap};
use crate::enums::field::FieldMark;
use crate::enums::player::Player;
use crate::enums::view_action::ViewAction;
use crate::services::game::Game;
use crate::traits::view_model::ViewModel;

pub struct AiGameView{
    game: Game,
    field_selection: u8,
    ai_thinking_gauge: u16,
}

impl AiGameView{

    pub fn new() -> AiGameView{
        AiGameView{
            game: Game::new(),
            field_selection: 7,
            ai_thinking_gauge: 0,
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
        let title = [
            Line::from("__   __                 _                    "),
            Line::from("\\ \\ / /__  _   _ _ __  | |_ _   _ _ __ _ __  "),
            Line::from(" \\ V / _ \\| | | | '__| | __| | | | '__| '_ \\ "),
            Line::from("  | | (_) | |_| | |    | |_| |_| | |  | | | |"),
            Line::from("  |_|\\___/ \\__,_|_|     \\__|\\__,_|_|  |_| |_|"),
        ];
        Text::from_iter(title).style(Self::get_player_style())
    }

    fn get_enemy_turn_text<'a>() -> Text<'a>{
        let title = [
            Line::from(" _____                              _                    "),
            Line::from("| ____|_ __   ___ _ __ ___  _   _  | |_ _   _ _ __ _ __  "),
            Line::from("|  _| | '_ \\ / _ \\ '_ ` _ \\| | | | | __| | | | '__| '_ \\ "),
            Line::from("| |___| | | |  __/ | | | | | |_| | | |_| |_| | |  | | | |"),
            Line::from("|_____|_| |_|\\___|_| |_| |_|\\__, |  \\__|\\__,_|_|  |_| |_|"),
            Line::from("                            |___/                        ")
        ];
        Text::from_iter(title).style(Self::get_opponent_style())
    }

    fn get_win_text<'a>() -> Text<'a>{
        let title = [
            Line::from("__   __                                "),
            Line::from("\\ \\ / /__  _   _  __      _____  _ __  "),
            Line::from(" \\ V / _ \\| | | | \\ \\ /\\ / / _ \\| '_ \\ "),
            Line::from("  | | (_) | |_| |  \\ V  V / (_) | | | |"),
            Line::from("  |_|\\___/ \\__,_|   \\_/\\_/ \\___/|_| |_|"),
        ];
        Text::from_iter(title).style(Style::new().green())
    }

    fn get_lost_text<'a>() -> Text<'a>{
        let title = [
            Line::from("__   __            _           _   "),
            Line::from("\\ \\ / /__  _   _  | | ___  ___| |_ "),
            Line::from(" \\ V / _ \\| | | | | |/ _ \\/ __| __|"),
            Line::from("  | | (_) | |_| | | | (_) \\__ \\ |_ "),
            Line::from("  |_|\\___/ \\__,_| |_|\\___/|___/\\__|"),
        ];
        Text::from_iter(title).style(Style::new().red())
    }

    fn get_draw_text<'a>() -> Text<'a>{
        let title = [
            Line::from(" ____  ____      ___        __"),
            Line::from("|  _ \\|  _ \\    / \\ \\      / /"),
            Line::from("| | | | |_) |  / _ \\ \\ /\\ / / "),
            Line::from("| |_| |  _ <  / ___ \\ V  V /"),
            Line::from("|____/|_| \\_\\/_/   \\_\\_/\\_/   ")
        ];
        Text::from_iter(title).style(Style::new().gray())
    }

    fn get_x<'a>() -> Text<'a>{
        let x = [
            Line::from("__  __"),
            Line::from("\\ \\/ /"),
            Line::from(" >  < "),
            Line::from("/_/\\_\\"),
        ];

        Text::from_iter(x)
    }

    fn get_o<'a>() -> Text<'a>{
        let o = [
            Line::from("  ___  "),
            Line::from(" / _ \\ "),
            Line::from("| (_) |"),
            Line::from(" \\___/ "),
        ];
        Text::from_iter(o)
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

    fn get_controls_text<'a>() -> Text<'a>{
        let controls: Vec<Line> = vec![
            Line::from(vec![Span::styled("←,↑,↓,→",Style::new().fg(Color::Magenta)),Span::styled(" - select tile",Style::new())]),
            Line::from(""),
            Line::from(vec![Span::styled("Enter",Style::new().fg(Color::Magenta)),Span::styled(" - confirm selection",Style::new())]),
            Line::from(""),
            Line::from(vec![Span::styled("ESC",Style::new().fg(Color::Magenta)),Span::styled(" - exit the game",Style::new())])
        ];
        Text::from(controls)
    }

    fn get_smiley_face<'a>() -> Text<'a>{
        let face = [
            Line::from("     ██      ██     "),
            Line::from("     ██      ██     "),
            Line::from("                    "),
            Line::from("██                ██"),
            Line::from(" ██              ██ "),
            Line::from("  ████████████████  ")
        ];
        Text::from_iter(face)
    }

    fn get_thinking_face<'a>() -> Text<'a>{
        let face = [
            Line::from("                 ███"),
            Line::from("                   █"),
            Line::from("    ██   ██      ███"),
            Line::from("    ██   ██      █  "),
            Line::from("                    "),
            Line::from("                 █  "),
            Line::from("  ████████████      ")
        ];
        Text::from_iter(face)
    }

    fn get_happy_face<'a>() -> Text<'a>{
        let face = [
            Line::from("   ███    ███   "),
            Line::from("   ███    ███   "),
            Line::from("                "),
            Line::from("█              █"),
            Line::from("███          ███"),
            Line::from("  ████████████  ")
        ];
        Text::from_iter(face)
    }

    fn get_angry_face<'a>() -> Text<'a>{
        let face = [
            Line::from("    █      █    "),
            Line::from("     █    █     "),
            Line::from("   ██ █  █ ██   "),
            Line::from("   ██      ██   "),
            Line::from("                "),
            Line::from(" ████ ██████  ██"),
            Line::from("██  █████  ████ "),
        ];
        Text::from_iter(face)
    }

    fn draw_ai_status(&mut self, frame: &mut Frame, right_top: Rect){
        let right_top_separation = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(9), Constraint::Fill(1),  Constraint::Length(1)])
            .split(right_top.inner(Margin::new(1,1)));

        self.draw_ai_face(right_top_separation[0],frame);

        let text = Paragraph::new("Waiting for turn...");
        frame.render_widget(text.centered(),right_top_separation[1].centered_vertically(Constraint::Length(1)));

        self.draw_gauge_and_advance_thinking(right_top_separation[2],frame);
    }

    fn draw_ai_face(&mut self, area: Rect, frame: &mut Frame){
        let mut face= Self::get_smiley_face();
        if(self.game.winner.is_none()){
            if(self.game.current_player == Player::Player){
                face = Self::get_smiley_face();
            }
            else{
                face = Self::get_thinking_face();
            }
        }
        else{
            if let Some(winner) = self.game.winner{
                if(winner == self.game.opponent_mark){
                    face = Self::get_happy_face();
                }
                else{
                    face = Self::get_angry_face();
                }
            }
        }

        let face = face.style(Self::get_opponent_style());
        frame.render_widget(face.centered(),area.inner(Margin::new(0,1)));
    }

    fn draw_gauge_and_advance_thinking(&mut self, area: Rect, frame:&mut Frame){
        if(self.game.current_player == Player::Opponent && self.game.winner.is_none()){
            let gauge = Gauge::default()
                .percent(self.ai_thinking_gauge)
                .style(Self::get_opponent_style());
            frame.render_widget(gauge, area.centered_vertically(Constraint::Length(1)));
            self.update_ai_gauge_and_wait(10);
        }
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

            frame.render_widget(Paragraph::new(mark_art).centered().block(block),field);

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
        if(self.field_selection < 9){
            if let Err(message) = self.game.make_move(self.field_selection){
                self.draw_error_text(message);
            }
        }
    }

    fn opponent_make_move(&mut self){
        if(self.game.winner.is_none() && self.game.current_player == Player::Opponent && self.ai_thinking_gauge == 100){
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
            self.ai_thinking_gauge = 0;
        }

    }

    fn update_ai_gauge_and_wait(&mut self, amount: u16){
        self.ai_thinking_gauge += amount;
        thread::sleep(Duration::from_millis(100));
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


        //Drawing left/right separation
        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Game"), left_area);

        let separated_right_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(right_area);
        let right_top = separated_right_area[0];
        let right_bottom = separated_right_area[1];

        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("AI Status").title_alignment(Alignment::Center), right_top);
        frame.render_widget(Block::new().borders(Borders::ALL).border_type(BorderType::Rounded).title("Controls").title_alignment(Alignment::Center), right_bottom);

        //Drawing ai status
        self.draw_ai_status(frame,right_top);

        //Drawing right screen controls
        let controls_rect = right_bottom.inner(Margin::new(2,2));

        let controls_paragraph = Paragraph::new(Self::get_controls_text()).wrap(Wrap { trim: true });
        frame.render_widget(controls_paragraph.centered(), controls_rect);

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
        return if(self.game.winner.is_some()){
            self.handle_input_end(key)
        }else if(self.game.current_player == Player::Player){
            self.handle_input_your_turn(key)
        }else {
            self.handle_input_enemy_turn(key)
        };

    }

    fn additional_actions(&mut self) {
        self.opponent_make_move();
    }
}



