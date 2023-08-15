use std::{io::{self, Stdout}};

use _2048_rs::{Arrow, Board};
use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, EnterAlternateScreen, enable_raw_mode, LeaveAlternateScreen}, execute,
};

use ratatui::{prelude::*, widgets::*};

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let mut rng = rand::thread_rng();
    let mut board = Board::new(&mut rng);
    loop {
        print_board(board, terminal)?;
        let input = crossterm::event::read()?;
        match input {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q' | 'Q'),
                kind: KeyEventKind::Press,
                ..
            }) => break Ok(()),
            Event::Key(KeyEvent { code, kind: KeyEventKind::Press, ..}) => {
                let Ok(direction) = Arrow::try_from(code) else {
                    continue;
                };
                board.play_changed(direction, &mut rng);
            }
            _ => continue,
        }
    }
}

fn print_board(board: Board, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    terminal.draw(|frame|{
        let table = Into::<Table<'static>>::into(board);
        frame.render_widget(table, Rect { x: 0, y: 0, width: 30, height: 12 })
    })?;
    Ok(())
}
