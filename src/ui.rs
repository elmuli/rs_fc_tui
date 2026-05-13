use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect, Margin};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};
use ratatui::text::{Line, Text};

use crate::app;

pub fn render(frame: &mut Frame, app: &app::App) {
    draw_layout(frame, app);
}

fn draw_layout(frame: &mut Frame, app: &app::App) {
    use Constraint::{Fill, Length, Min};

    let [title_area, main_area, status_area] = Layout::vertical([Length(1), Min(0), Length(1)]).areas(frame.area());
    let [left_area, right_area] = Layout::horizontal([Length(20), Fill(1)]).areas(main_area);

    frame.render_widget(Block::bordered().title("Flascard TUI".yellow().bold()), title_area);
    frame.render_widget(Block::bordered().title("Left Area").blue(), left_area);
    frame.render_widget(Block::bordered().title("?: help").yellow(), status_area);

    let right_block = Block::bordered().title("Card Area".italic()).green();
    let inner_area = right_block.inner(right_area);
    frame.render_widget(right_block, right_area);

    let text = Text::from(vec![
        Line::from("水".bold().yellow()),
        Line::from("みず".italic().cyan()),
        Line::from("water".dim()),
    ]);
    let lines = text.height() as u16;

    let [_, center_area, _] = Layout::vertical([Fill(1),Length(lines),Fill(1)]).areas(inner_area);
    let paragraph = Paragraph::new(text).centered();
    frame.render_widget(paragraph, center_area);

    if app.is_help {
        draw_help_screen(frame);
    }
}

fn draw_help_screen(frame: &mut Frame){
    use Constraint::{Fill, Length, Min};

    let center_area = Rect::new(frame.area().width/3, frame.area().height/3, 2*frame.area().width/3, 2*frame.area().height/3);

    let help_block = Block::bordered().title("Help");
    let inner_area = center_area.inner(Margin::new(2,1));
    frame.render_widget(help_block, center_area);

    let text = Text::from(vec![
        Line::from("q:  quit".bold().yellow()),
        Line::from("?:  help".italic().cyan()),
    ]);
    let lines = text.height() as u16;


    let paragraph = Paragraph::new(text).centered();
    frame.render_widget(paragraph, inner_area);
}
