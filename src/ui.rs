

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let block = Block::bordered()
        .title("My TUI");

    let paragraph = Paragraph::new("Hello from the terminal")
        .block(block);

    frame.render_widget(paragraph, area);
}
