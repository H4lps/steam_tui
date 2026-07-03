

fn main() {
    let mut terminal = ratatui::init();
    let app = App::new();

    terminal.draw(|frame| {
        ui(frame,&app);
    })?;

    ratatui::restore();
    Ok(())
}
