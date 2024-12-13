use ratatui::{
    style::{ Style, Stylize, palette::tailwind::{ GREEN, RED } },
    widgets::{ Block, BorderType, Paragraph, Widget },
    prelude::{ Buffer, Line, Rect },
    text::Span,
};

/// Terminal too small widget.
#[derive(Debug, Default)]
pub struct TerminalTooSmall {}

impl Widget for TerminalTooSmall {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title_top(Line::from(" Terminal size too small ").centered().bold())
            .border_type(BorderType::Rounded);

        Paragraph::new(
            vec![
                Line::from(""),
                Line::from("Please resize your terminal to at least 106x24".bold()),
                Line::from(""),
                Line::from(
                    vec![
                        Span::raw("Width = "),
                        Span::styled(
                            format!("{} ", area.width),
                            Style::new().fg(if area.width >= 106 { GREEN.c500 } else { RED.c500 })
                        ),
                        Span::raw("Height = "),
                        Span::styled(
                            format!("{} ", area.height),
                            Style::new().fg(if area.height >= 24 { GREEN.c500 } else { RED.c500 })
                        )
                    ]
                )
            ]
        )
            .centered()
            .block(block)
            .render(area, buf);
    }
}
