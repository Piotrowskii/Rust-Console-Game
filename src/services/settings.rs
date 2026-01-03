use ratatui::style::Color;

pub struct Settings{
    pub player_color: Color,
    pub opponent_color: Color,
}

impl Settings{
    pub fn new() -> Settings{
        Settings{
            player_color: Color::Blue,
            opponent_color: Color::Red,
        }
    }

    pub fn change_player_style(&mut self,color: Color){
        self.player_color = color;
    }

    pub fn change_opponent_style(&mut self,color: Color){ self.opponent_color = color; }
}