use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod ui;

fn main() -> std::io::Result<()> {
    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(|frame| ui::render(frame))?;
            if handle_events()? {
                break Ok(());
            }
        }
    })
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}
