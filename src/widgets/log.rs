use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{ Line, Text },
    widgets::{ Block, Borders, Paragraph, Widget },
};

/// Log widget.
#[derive(Debug)]
pub struct Log {
    /// Log vector to render.
    pub log: Vec<String>,
}

impl Widget for Log {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Text::from(self.log.into_iter().rev().collect::<Vec<String>>().join("\n")))
            .block(
                Block::default()
                    .title_top(Line::from("Log").centered().bold())
                    .borders(Borders::ALL)
                    .border_set(border::ROUNDED)
            )
            .render(area, buf);
    }
}
