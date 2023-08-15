use std::{
    io::{self, Stdout},
    path::Path,
};

use directories::ProjectDirs;

use _2048_rs::{load, print_board, save, Arrow, Board};
use anyhow::Result;
use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::*;

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let score = run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    println!("score: {score}");
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<u64> {
    let mut rng = rand::thread_rng();
    let mut board = Board::new(&mut rng);

    // if project specified directory is not avaliable,
    // just place saves at ./saves/saveX
    let project_dir = ProjectDirs::from("moe", "Meowkatee", "2048-rs");
    let data_dir = project_dir
        .as_ref()
        .map(|proj| proj.data_dir())
        .unwrap_or(&Path::new("saves"));
    loop {
        print_board(board, terminal, board.is_lost(&mut rng))?;
        let input = crossterm::event::read()?;
        match input {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q' | 'Q'),
                kind: KeyEventKind::Press,
                ..
            }) => break Ok(board.score()),
            Event::Key(KeyEvent {
                code: KeyCode::Char('s' | 'S'),
                kind: KeyEventKind::Release,
                ..
            }) => {
                terminal.clear()?;
                save(board, data_dir)?;
                terminal.clear()?;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('l' | 'L'),
                kind: KeyEventKind::Release,
                ..
            }) => {
                terminal.clear()?;
                load(&mut board, data_dir)?;
                terminal.clear()?;
            }
            Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) => {
                let Ok(direction) = Arrow::try_from(code) else {
                    continue;
                };
                board.play_changed(direction, &mut rng);
            }
            _ => continue,
        }
    }
}
