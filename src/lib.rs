use std::num::NonZeroU8;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    board: [[Option<NonZeroU8>; 4]; 4],
}

impl Board {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut initial_board = [[None; 4]; 4];
        let indice = (0..4)
            .map(|i| (0..4).map(move |j| (i, j)))
            .flatten()
            .collect::<Vec<_>>();
        let posi = indice.choose_multiple(rng, 2);
        posi.for_each(|&(x, y)| initial_board[x][y] = NonZeroU8::new(1));
        initial_board.into()
    }

    pub fn play_changed(&mut self, direction: Arrow, rng: &mut ThreadRng) -> bool {
        let prev_state = self.clone();
        self.merge(direction);
        let changed = prev_state != *self;
        if changed {
            self.gen_num(rng);
        }
        changed
    }

    pub fn gen_num(&mut self, rng: &mut ThreadRng) -> bool {
        if self.is_full() {
            return false;
        }

        let &(x, y) = (0..4)
            .map(|i| (0..4).map(move |j| (i, j)))
            .flatten()
            .filter(|&(x, y)| self.board[x][y].is_none())
            .collect::<Vec<_>>()
            .choose(rng)
            .unwrap();

        self.board[x][y] = if rng.gen_ratio(1, 10) {
            NonZeroU8::new(2)
        } else {
            NonZeroU8::new(1)
        };

        true
    }

    fn is_full(&self) -> bool {
        self.board
            .iter()
            .map(|row| row.iter())
            .flatten()
            .all(Option::is_some)
    }

    pub fn is_lost(&self, rng: &mut ThreadRng) -> bool {
        Arrow::iter()
            .into_iter()
            .all(|op| !self.clone().play_changed(op, rng))
    }
}

impl Board {
    fn scan(
        &mut self,
        direction: Arrow,
        op: impl Fn(&mut Option<NonZeroU8>, &mut Option<NonZeroU8>),
    ) {
        match direction {
            Arrow::Up => (0..3).rev().for_each(|x| {
                (0..4).map(|y| (x, y)).for_each(|(x, y)| {
                    let (above, below) = self.board.split_at_mut(x + 1);
                    let (above, below) = (
                        &mut above.last_mut().unwrap()[y],
                        &mut below.first_mut().unwrap()[y],
                    );
                    op(above, below);
                })
            }),
            Arrow::Down => (0..3).for_each(|x| {
                (0..4).map(|y| (x, y)).for_each(|(x, y)| {
                    let (above, below) = self.board.split_at_mut(x + 1);
                    let (above, below) = (
                        &mut above.last_mut().unwrap()[y],
                        &mut below.first_mut().unwrap()[y],
                    );
                    op(above, below);
                })
            }),
            Arrow::Left => (0..4).for_each(|x| {
                (0..3).rev().map(|y| (x, y)).for_each(|(x, y)| {
                    let (left, right) = self.board[x].split_at_mut(y + 1);
                    let (left, right) = (left.last_mut().unwrap(), right.first_mut().unwrap());
                    op(left, right);
                })
            }),
            Arrow::Right => (0..4).for_each(|x| {
                (0..3).rev().map(|y| (x, y)).for_each(|(x, y)| {
                    let (left, right) = self.board[x].split_at_mut(y + 1);
                    let (left, right) = (left.last_mut().unwrap(), right.first_mut().unwrap());
                    op(left, right);
                })
            }),
        }
    }

    fn merge(&mut self, direction: Arrow) {
        self.squash(direction);

        match direction {
            Arrow::Up => self.scan(direction, |above, below| {
                if above.is_some() && above == below {
                    *below = above.unwrap().checked_add(1);
                    *above = None;
                }
            }),
            Arrow::Down => self.scan(direction, |above, below| {
                if above.is_some() && above == below {
                    *above = below.unwrap().checked_add(1);
                    *below = None;
                }
            }),
            Arrow::Left => self.scan(direction, |left, right| {
                if right.is_some() && left == right {
                    *right = left.unwrap().checked_add(1);
                    *left = None;
                }
            }),
            Arrow::Right => self.scan(direction, |left, right| {
                if right.is_some() && left == right {
                    *left = right.unwrap().checked_add(1);
                    *right = None;
                }
            }),
        }

        self.squash(direction);
    }

    fn squash_once(&mut self, direction: Arrow) {
        match direction {
            Arrow::Up => self.scan(direction, |above, below| {
                if above.is_none() && below.is_some() {
                    *above = below.take();
                }
            }),
            Arrow::Down => self.scan(direction, |above, below| {
                if below.is_none() && above.is_some() {
                    *below = above.take();
                }
            }),
            Arrow::Left => self.scan(direction, |left, right| {
                if left.is_none() && right.is_some() {
                    *left = right.take();
                }
            }),
            Arrow::Right => self.scan(direction, |left, right| {
                if right.is_none() && left.is_some() {
                    *right = left.take();
                }
            }),
        }
    }

    fn squash(&mut self, direction: Arrow) {
        for _ in 0..3 {
            self.squash_once(direction);
        }
    }
}

impl From<[[Option<NonZeroU8>; 4]; 4]> for Board {
    fn from(value: [[Option<NonZeroU8>; 4]; 4]) -> Self {
        Self { board: value }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

impl Arrow {
    fn iter() -> [Self; 4] {
        [Arrow::Up, Arrow::Down, Arrow::Left, Arrow::Right]
    }
}
