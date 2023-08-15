use std::num::NonZeroU8;

use ratatui::widgets::*;
use ratatui::prelude::*;

use crate::Board;

fn color_of(state: NonZeroU8) -> Color {
    match state.get() % 8 {
        0 => Color::Rgb(255, 0, 77),
        1 => Color::Rgb(255, 163, 0),
        2 => Color::Rgb(255, 240, 36),
        3 => Color::Rgb(0, 231, 86),
        4 => Color::Rgb(41, 173, 255),
        5 => Color::Rgb(131, 118, 156),
        6 => Color::Rgb(255, 119, 168),
        7 => Color::Rgb(255, 119, 168),
        _ => unreachable!(),
    }
}

impl From<Board> for Table<'static> {
    fn from(value: Board) -> Self {
        Table::new(
            value
                .board
                .map(|row| Row::new(row.map(cell_to_widget)).height(2)),
        )
        .widths(&[Constraint::Percentage(25); 4])
        .column_spacing(0)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("2048-rs")
                .style(Style::new().bg(Color::DarkGray))
                .padding(Padding::new(1, 1, 1, 1)),
        )
    }
}

fn cell_to_widget(cell: Option<NonZeroU8>) -> Cell<'static> {
    let text = Text::raw(
        cell.map(|i| 2_u64.saturating_pow(i.get() as _).to_string())
            .unwrap_or_default(),
    );
    Cell::from(text).style(
        Style::new()
            .bg(Color::Black)
            .fg(cell.map(color_of).unwrap_or(Color::Black)),
    )
}