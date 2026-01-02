use std::time::Duration;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::Frame;
use crate::enums::view_action::ViewAction;

pub trait ViewModel{
    fn render_widgets(&mut self, frame: &mut Frame);
    fn handle_inputs(&mut self, key: KeyEvent) -> ViewAction;

    fn additional_actions(&mut self);

    fn handle_events(&mut self) -> Result<ViewAction, std::io::Error>{
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press{
                    return Ok(self.handle_inputs(key))
                }
            }
        }
        Ok(ViewAction::Nothing)
    }
    fn render(&mut self, frame: &mut Frame) -> ViewAction{
        self.render_widgets(frame);
        self.additional_actions();
        match self.handle_events(){
            Ok(value) => {value}
            Err(_) => { ViewAction::Quit }
        }
    }
}