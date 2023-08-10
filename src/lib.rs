use std::num::NonZeroU8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Board {
    board: [[Option<NonZeroU8>; 4]; 4],
}

impl Board {
    pub fn is_lost(&self) -> bool {
        self.board
            .iter()
            .map(|row| row.iter())
            .flatten()
            .all(Option::is_some)
            && !self.is_mergable()
    }

    fn is_mergable(&self) -> bool {
        let mergable_row = || {
            (0..4).any(|x| {
                (0..3).map(|y| (x, y)).any(|(x, y)| {
                    let left = self.board[x][y];
                    let right = self.board[x][y + 1];
                    left.is_some() && left == right
                })
            })
        };
        let mergable_col = || {
            (0..3).any(|x| {
                (0..4).map(|y| (x, y)).any(|(x, y)| {
                    let above = self.board[x][y];
                    let below = self.board[x + 1][y];
                    above.is_some() && above == below
                })
            })
        };
        mergable_row() || mergable_col()
    }

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

    pub fn merge(&mut self, direction: Arrow) {
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

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test_mergable() {
        let mergable_boards = [
            [
                [None; 4],
                [None; 4],
                [None, None, NonZeroU8::new(3), None],
                [None, None, NonZeroU8::new(3), None],
            ],
            [
                [None; 4],
                [None; 4],
                [None; 4],
                [None, None, NonZeroU8::new(1), NonZeroU8::new(1)],
            ],
        ];

        let unmergable_boards = [
            [
                [None; 4],
                [None; 4],
                [None, None, NonZeroU8::new(2), None],
                [None, None, NonZeroU8::new(3), None],
            ],
            [
                [None; 4],
                [None; 4],
                [None; 4],
                [None, None, NonZeroU8::new(2), NonZeroU8::new(1)],
            ],
            [[None; 4], [None; 4], [None; 4], [None; 4]],
        ];
        assert!(mergable_boards
            .into_iter()
            .all(|board| Board::from(board).is_mergable()));

        assert!(unmergable_boards
            .into_iter()
            .all(|board| !Board::from(board).is_mergable()));
    }

    #[test]
    fn test_is_lost() {
        let lost_boards = [
            [
                [NonZeroU8::new(1), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(2), NonZeroU8::new(1),NonZeroU8::new(2),NonZeroU8::new(1),],
                [NonZeroU8::new(1), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(2), NonZeroU8::new(1),NonZeroU8::new(2),NonZeroU8::new(1),],
            ],
        ];

        let not_yet_losts = [
            [
                [NonZeroU8::new(1), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(2), NonZeroU8::new(1),NonZeroU8::new(2),NonZeroU8::new(1),],
                [NonZeroU8::new(1), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(2), NonZeroU8::new(1),NonZeroU8::new(2),None],
            ],
            [

                [NonZeroU8::new(1), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(3), NonZeroU8::new(1),NonZeroU8::new(2),NonZeroU8::new(1),],
                [NonZeroU8::new(2), NonZeroU8::new(2),NonZeroU8::new(1),NonZeroU8::new(2),],
                [NonZeroU8::new(2), NonZeroU8::new(1),NonZeroU8::new(2),NonZeroU8::new(1),],
            ]
        ];

        assert!(lost_boards.into_iter().map(Board::from).all(|board| board.is_lost()));
        assert!(not_yet_losts.into_iter().map(Board::from).all(|board| !board.is_lost()));
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}
