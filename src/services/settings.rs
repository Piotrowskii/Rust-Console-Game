use ratatui::style::Style;

pub struct Settings{
    pub player_style: Style,
    pub opponent_style: Style,
}

impl Settings{
    pub fn new() -> Settings{
        Settings{
            player_style: Style::new().blue(),
            opponent_style: Style::new().red(),
        }
    }

    pub fn change_player_style(&mut self,style: Style){
        self.player_style = style;
    }

    pub fn change_opponent_style(&mut self,style: Style){
        self.opponent_style = style;
    }
}