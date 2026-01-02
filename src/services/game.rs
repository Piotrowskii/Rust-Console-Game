use crate::enums::field::FieldMark;
use crate::enums::player::Player;
use crate::helpers::vector_helper::VecExt;

pub struct Game{
    pub board: [FieldMark;9],
    pub current_player: Player,
    pub player_mark: FieldMark,
    pub opponent_mark: FieldMark,
    pub winner: Option<FieldMark>,
}

impl Game{
    pub fn new() -> Game{
        Game{
            board: [FieldMark::Empty; 9],
            current_player: Player::Player,
            player_mark: FieldMark::X,
            opponent_mark: FieldMark::O,
            winner: None
        }
    }

    pub fn make_move(&mut self, index: u8) -> Result<(),String>{
        if index >= 9 {
            return Err("Field number out of scope".to_string());
        }

        if self.board[index as usize] != FieldMark::Empty{
            return Err("Select empty field".to_string())
        }

        match self.current_player{
            Player::Player => {
                self.board[index as usize] = self.player_mark;
                self.check_if_game_already_won();
                self.current_player = Player::Opponent;
            }
            Player::Opponent => {
                self.board[index as usize] = self.opponent_mark;
                self.check_if_game_already_won();
                self.current_player = Player::Player;
            }
        }
        Ok(())
    }

    fn check_if_game_already_won(&mut self){
        let potential_winner: Option<FieldMark> = self.check_win(&self.board);
        if let Some(winner) = potential_winner{
            self.winner = Some(winner);
        }
    }


    pub fn get_ai_move(&self, player: Player) -> Option<u8>{
        let my_mark = if player == Player::Player {self.player_mark} else {self.opponent_mark};
        let enemy_mark = if player == Player::Player {self.opponent_mark} else {self.player_mark};
        let mut empty_spaces: Vec<u8> = Vec::new();


        //TODO:write it better
        //Attack
        for (i, mark) in self.board.iter().enumerate(){
            if(matches!(mark,FieldMark::Empty)){
                empty_spaces.push(i as u8);

                let mut board_copy = self.board.clone();
                board_copy[i] = my_mark.clone();
                if let Some(winning_mark) = self.check_win(&board_copy){
                    if winning_mark == my_mark{
                        return Some(i as u8);
                    }
                }
            }
        }

        //Defense
        for (i, mark) in self.board.iter().enumerate() {
            if (matches!(mark,FieldMark::Empty)) {
                let mut board_copy = self.board.clone();
                board_copy[i] = enemy_mark.clone();
                if let Some(winning_mark) = self.check_win(&board_copy){
                    if winning_mark != my_mark{
                        return Some(i as u8);
                    }
                }
            }
        }
        empty_spaces.random()
    }

    fn check_win(&self, board: &[FieldMark; 9]) -> Option<FieldMark>{
        for i in 0..=2{
            if(board[0+(3*i)] == board[1+(3*i)] && board[1+(3*i)] == board[2+(3*i)] && board[2+(3*i)] != FieldMark::Empty){
                return Some(board[0+(3*i)]);
            }
            else if(board[0+i] == board[3+i] && board[3+i] == board[6+i] && board[6+i] != FieldMark::Empty){
                return Some(board[0+i]);
            }
        }

        for i in 0..=1{
            if(board[0 + (i*2)] == board[4] && board[4] == board[8 - (i*2)] && board[8- (i*2)] != FieldMark::Empty){
                return Some(board[4]);
            }
        }

        if board.iter().filter(|&mark| *mark != FieldMark::Empty).count() == 9{
            return Some(FieldMark::Empty);
        }

        None
    }
}