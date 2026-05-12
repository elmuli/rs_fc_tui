use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::widgets::{Block, Paragraph};
use ratatui::text::{Line, Text};

pub fn render(frame: &mut Frame) {
    draw_layout(frame);
}

fn draw_layout(frame: &mut Frame) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [title_area, main_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);


    frame.render_widget(Block::bordered().title("Flascard TUI".yellow().bold()), title_area);
    frame.render_widget(Block::bordered().title("Left Area").blue(), left_area);


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
}
