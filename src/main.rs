use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

mod ui;
mod app;


fn main() -> std::io::Result<()> {
    let mut app = app::App{
        should_quit: false,
        is_help: false,
    };

    ratatui::run(|mut terminal| {
        loop {
            terminal.draw(|frame| ui::render(frame, &app))?;
            handle_events(&mut app);
            if app.should_quit {
                break Ok(());
            }
        }
    })
}

fn handle_events(app: &mut app::App) -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => app.should_quit = true,
            KeyCode::Char('?') => app.is_help = true,
            KeyCode::Char('c') => app.is_help = false,
            _ => {}
        },
      _ => {}
    }
    Ok(false)
}
