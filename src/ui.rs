
use ratatui::widgets::{Paragraph, Block,List};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use crate::app::App;
use std::fs;
use std::path::Path;

//need to fix relative pathing
const STEAM_ART_PATH: &str = "art/steam.txt";

pub fn ui(frame: &mut Frame, _app: &App) {
    let areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(frame.area());

    let games = _app.games();
    let game_names : Vec<&str> = games
        .iter()
        .map(|game| game.name())
        .collect();


    let block = Block::bordered()
        .title("Steam");
    
    let path = Path::new(STEAM_ART_PATH);

    let steam_art = fs::read_to_string(path)
        .expect("Error reading steam art file");

    let paragraph = Paragraph::new(steam_art)
        .block(block);
    

    let list = List::new(game_names)
        .block(Block::bordered()
            .title("Games")
        );


    frame.render_widget(paragraph, areas[1]);
    frame.render_widget(list, areas[0]);
}
/*
fn draw_game(block: &block, name: &str) {
 
}
*/
