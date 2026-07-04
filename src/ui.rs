
use ratatui::widgets::{Paragraph, Block};
use ratatui::Frame;
use crate::app::App;

pub fn ui(frame: &mut Frame, _app: &App) {
    let area = frame.area();

    let block = Block::bordered()
        .title("My TUI");

    let paragraph = Paragraph::new("Hello from the terminal")
        .block(block);

    frame.render_widget(paragraph, area);
}
