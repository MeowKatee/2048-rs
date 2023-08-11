use std::num::NonZeroU8;

use _2048_rs::{Arrow, Board};
use crossterm::style::Color;

fn main() {
    let mut rng = rand::thread_rng();
    let board = Board::new(&mut rng);

    loop {}
}

fn color_of(state: NonZeroU8) -> Color {
    match state.get() % 15 {
        0 => Color::Yellow,
        1 => Color::Blue,
        2 => Color::Green,
        3 => Color::Red,
        4 => Color::DarkYellow,
        5 => Color::DarkBlue,
        6 => Color::DarkRed,
        7 => Color::Magenta,
        8 => Color::White,
        _ => unreachable!(),
    }
}

fn print_board(board: &Board) {

}