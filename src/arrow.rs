use crossterm::event::KeyCode;
use anyhow::Result;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

impl Arrow {
    pub fn iter() -> [Self; 4] {
        [Arrow::Up, Arrow::Down, Arrow::Left, Arrow::Right]
    }
}

impl TryFrom<KeyCode> for Arrow {
    type Error = ();

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyCode::Up => Arrow::Up,
            KeyCode::Down => Arrow::Down,
            KeyCode::Left => Arrow::Left,
            KeyCode::Right => Arrow::Right,
            _ => Err(())?,
        })
    }
}
