mod app;
mod ui;

use app::App;
use ui::ui;
use std::path::PathBuf;
use crossterm::event::{self, Event, KeyCode, KeyModifiers};


fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    //Placeholder config
    let personal_path = 
        PathBuf::from(r"/home/liampatel/.steam/steam/steamapps");
    let selected_index = 0;
    let search = "".to_string();
    let should_quit = false;

    let steam = App::new(
        personal_path,
        selected_index,
        search,
        should_quit,
    )?;

    

    loop {    
        //Base Case
       if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('c')
                && key.modifiers.contains(KeyModifiers::CONTROL)
            {
                break;
            }
        }

        terminal.draw(|frame| {
            ui(frame,&steam);
        })?;     
    }
    ratatui::restore();
    Ok(())
}
