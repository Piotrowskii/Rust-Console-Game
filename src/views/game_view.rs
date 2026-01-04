use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::time::Duration;
use ratatui::crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Gauge, Paragraph, Wrap};
use crate::enums::field::FieldMark;
use crate::enums::player::Player;
use crate::enums::player_type::PlayerType;
use crate::enums::view_action::ViewAction;
use crate::services::game::Game;
use crate::traits::view_model::ViewModel;
use crate::services::game_art as Art;
use crate::services::settings::Settings;

pub struct AiGameView{
    game: Game,
    opponent_type: PlayerType,
    settings: Rc<RefCell<Settings>>,
    field_selection: u8,
    ai_thinking_gauge: u16,
}

impl AiGameView{

    pub fn new(opponent_type: PlayerType, settings: Rc<RefCell<Settings>>) -> AiGameView{
        AiGameView{
            game: Game::new(),
            field_selection: 7,
            ai_thinking_gauge: 0,
            opponent_type,
            settings,
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


    fn get_field_mark_art(&self, field_mark: FieldMark) -> Text<'static>{
        let mark = match field_mark {
            FieldMark::X => {Art::x()},
            FieldMark::O => {Art::o()}
            FieldMark::Empty => {Text::from("")}
        };
        let style = self.get_style_by_mark(field_mark);
        mark.style(style)
    }

    fn get_controls_text() -> Text<'static>{
        let controls: Vec<Line> = vec![
            Line::from(vec![Span::styled("←,↑,↓,→",Style::new().fg(Color::Magenta)),Span::styled(" - select tile",Style::new())]),
            Line::from(""),
            Line::from(vec![Span::styled("Enter",Style::new().fg(Color::Magenta)),Span::styled(" - confirm selection",Style::new())]),
            Line::from(""),
            Line::from(vec![Span::styled("ESC",Style::new().fg(Color::Magenta)),Span::styled(" - exit the game",Style::new())])
        ];
        Text::from(controls)
    }


    fn draw_ai_status(&mut self, frame: &mut Frame, right_top: Rect){
        if(self.opponent_type != PlayerType::Ai){
            let right_top_middle = right_top.centered_vertically(Constraint::Length(3)).inner(Margin::new(1,1));
           frame.render_widget(Paragraph::new("OFFLINE").centered(),right_top_middle);
        }
        else{
            let right_top_separation = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(9), Constraint::Fill(1),  Constraint::Length(1)])
                .split(right_top.inner(Margin::new(1,1)));

            self.draw_ai_face_and_text(right_top_separation[1],right_top_separation[0],frame);

            self.draw_ai_gauge_and_advance_thinking(right_top_separation[2],frame);
        }

    }
    fn draw_ai_face_and_text(&mut self, text_area: Rect, face_area: Rect, frame: &mut Frame){
        let mut face = Art::smiley_face();
        let mut ai_text = "waiting for turn..";
        if(self.game.winner.is_none()){
            if(self.game.current_player == Player::Player){
                face = Art::smiley_face();
                ai_text = "waiting for turn..";
            }
            else{
                face = Art::thinking_face();
                ai_text = "calculating move";
            };
        }
        else if let Some(winner) = self.game.winner{
            if(winner == self.game.opponent_mark){
                face = Art::happy_face();
                ai_text = "Yay i won, you suck";
            }
            else if(winner == FieldMark::Empty){
                face = Art::angry_face();
                ai_text = "You are as bad as me";
            }
            else{
                face = Art::angry_face();
                ai_text = "I will remember that";
            }
        }


        let face = face.style(self.get_opponent_style());
        frame.render_widget(face.centered(),face_area.inner(Margin::new(0,1)));

        frame.render_widget(Paragraph::new(ai_text).centered(),text_area.centered_vertically(Constraint::Length(1)));
    }

    fn draw_ai_gauge_and_advance_thinking(&mut self, area: Rect, frame:&mut Frame){
        if(self.game.current_player == Player::Opponent && self.game.winner.is_none()){
            let gauge = Gauge::default()
                .percent(self.ai_thinking_gauge)
                .style(self.get_opponent_style());
            frame.render_widget(gauge, area.centered_vertically(Constraint::Length(1)));
            self.update_ai_gauge_and_wait(10);
        }
    }

    fn update_ai_gauge_and_wait(&mut self, amount: u16){
        self.ai_thinking_gauge += amount;
        thread::sleep(Duration::from_millis(100));
    }

    fn get_player_style(&self) -> Style{
        Style::new().fg(self.settings.borrow().player_color)
    }

    fn get_opponent_style(&self) -> Style{
        Style::new().fg(self.settings.borrow().opponent_color)
    }

    fn get_style_by_mark(&self, field_mark: FieldMark) -> Style{
        if(self.game.player_mark == field_mark){
            self.get_player_style()
        }else if(self.game.opponent_mark == field_mark){
            self.get_opponent_style()
        }else{
            Style::new().gray()
        }
    }

    fn get_top_text(&self) -> Text<'_>{
        if(self.opponent_type == PlayerType::Ai){
            self.get_top_text_ai_game()
        }else{
            self.get_top_text_human_game()
        }
    }

    fn get_top_text_ai_game(&self) -> Text<'_>{
        match self.game.winner {
            None => {
                match self.game.current_player {
                    Player::Player => {Art::your_turn().style(self.get_player_style())}
                    Player::Opponent => {Art::enemy_turn().style(self.get_opponent_style())}
                }
            }
            Some(winner_mark) => {
                if(winner_mark == self.game.player_mark){
                    Art::you_won().style(Style::new().green())
                }else if(winner_mark == self.game.opponent_mark){
                    Art::you_lost().style(Style::new().red())
                }
                else{
                    Art::draw()
                }
            }
        }
    }

    fn get_top_text_human_game(&self) -> Text<'_> {
        let style = if self.game.current_player == Player::Player {self.get_player_style()} else {self.get_opponent_style()};

        match self.game.winner {
            None => {
                if(self.get_current_mark() == FieldMark::X){
                    Art::cross_turn().style(style)
                }else{
                    Art::circle_turn().style(style)
                }
            }
            Some(winner_mark) => {
                let style = self.get_style_by_mark(winner_mark);
                match winner_mark {
                    FieldMark::X => {Art::cross_won().style(style)}
                    FieldMark::O => {Art::circle_won().style(style)}
                    FieldMark::Empty => {Art::draw().style(style)}
                }
            }
        }
    }

    fn get_current_mark(&self) -> FieldMark{
        if self.game.current_player == Player::Player {self.game.player_mark} else {self.game.opponent_mark}
    }

    fn draw_board_tiles(&self, frame: &mut Frame, board_tiles: &[Rect]){
        assert_eq!(board_tiles.len(), self.game.board.len());

        for (i, mark) in self.game.board.iter().enumerate(){
            let field = board_tiles[i];
            let mut block = Block::new().borders(Borders::ALL).border_type(BorderType::Rounded);

            if(i == self.field_selection as usize && self.game.winner.is_none()){
                block = self.color_board_tile(block);
            }

            let mark_art = self.get_field_mark_art(*mark);

            frame.render_widget(Paragraph::new(mark_art).centered().block(block),field);

        }

    }

    fn color_board_tile<'a>(&self, block: Block<'a>) -> Block<'a>{
        if(self.opponent_type == PlayerType::Human){
            if(self.game.current_player == Player::Player){
                block.style(self.get_player_style())
            }else{
                block.style(self.get_opponent_style())
            }
        }
        else if(self.game.current_player == Player::Player){
            block.style(self.get_player_style())
        }
        else{
            block
        }
    }

    fn draw_error_text(&self,text: String){

    }

    fn move_selection_up(&mut self){ if(self.field_selection > 2){self.field_selection -= 3;} }

    fn move_selection_down(&mut self){ if(self.field_selection < 6){self.field_selection += 3;} }

    fn move_selection_left(&mut self){
        if(self.field_selection > 0){self.field_selection -= 1;}
    }

    fn move_selection_right(&mut self){
        if(self.field_selection < 8){self.field_selection += 1;}
    }

    fn player_make_move(&mut self){
        if self.field_selection < 9 && let Err(message) = self.game.make_move(self.field_selection){
            self.draw_error_text(message);
        }
    }

    fn ai_make_move(&mut self){
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

    fn opponent_make_move(&mut self){
        self.ai_make_move();
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
        if(self.game.winner.is_some()){
            self.handle_input_end(key)
        }else if(self.game.current_player == Player::Player){
            self.handle_input_your_turn(key)
        }else {
            if(self.opponent_type == PlayerType::Ai){
                self.handle_input_enemy_turn(key)
            }else{
                self.handle_input_your_turn(key)
            }

        }

    }

    fn additional_actions(&mut self) {
        if(self.opponent_type == PlayerType::Ai)
        {
            self.opponent_make_move();
        }
    }
}



