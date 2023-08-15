mod tests {
    #![allow(unused_imports)]
    use crate::*;
    #[test]
    fn test_is_lost() {
        let mut rng = rand::thread_rng();
        let lost_boards = [[
            [
                NonZeroU8::new(1),
                NonZeroU8::new(2),
                NonZeroU8::new(1),
                NonZeroU8::new(2),
            ],
            [
                NonZeroU8::new(2),
                NonZeroU8::new(1),
                NonZeroU8::new(2),
                NonZeroU8::new(1),
            ],
            [
                NonZeroU8::new(1),
                NonZeroU8::new(2),
                NonZeroU8::new(1),
                NonZeroU8::new(2),
            ],
            [
                NonZeroU8::new(2),
                NonZeroU8::new(1),
                NonZeroU8::new(2),
                NonZeroU8::new(1),
            ],
        ]];

        let not_yet_losts = [
            [
                [
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                ],
                [
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                ],
                [
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                ],
                [
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    None,
                ],
            ],
            [
                [
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                ],
                [
                    NonZeroU8::new(3),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                ],
                [
                    NonZeroU8::new(2),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                ],
                [
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                    NonZeroU8::new(2),
                    NonZeroU8::new(1),
                ],
            ],
        ];

        lost_boards
            .into_iter()
            .map(Board::from)
            .for_each(|board| assert!(board.is_lost(&mut rng)));
        not_yet_losts
            .into_iter()
            .map(Board::from)
            .for_each(|board| assert!(!board.is_lost(&mut rng)));
    }

    #[test]
    fn test_merge_squash() {
        #[rustfmt::skip]
        let pairs = [
            (
                [
                    [0, 0, 0, 1],
                    [0, 0, 3, 0],
                    [0, 0, 3, 0],
                    [0, 0, 3, 1]
                ],
                Arrow::Down,
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 3, 0],
                    [0, 0, 4, 2]
                ],
            ),
            (
                [
                    [0;4],
                    [0;4],
                    [0;4],
                    [4;4],
                ],
                Arrow::Right,
                [
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 0, 0],
                    [0, 0, 5, 5],
                ],
            ),
            (
                [
                    [0,0,0,0],
                    [0,0,0,0],
                    [0,0,0,0],
                    [1,0,1,1],
                ],
                Arrow::Left,
                [
                    [0,0,0,0],
                    [0,0,0,0],
                    [0,0,0,0],
                    [2,1,0,0],
                ],
            ),
            (
                [
                    [0,0,0,0],
                    [0,0,0,0],
                    [0,0,0,0],
                    [1,0,1,1],
                ],
                Arrow::Right,
                [
                    [0,0,0,0],
                    [0,0,0,0],
                    [0,0,0,0],
                    [0,0,1,2],
                ],
            ),
        ];
        pairs
            .into_iter()
            .enumerate()
            .map(|(i, (left, op, right))| (i, Board::from(left), op, Board::from(right)))
            .for_each(|(i, mut left, op, right)| {
                left.merge(op);
                assert!(left.board == right.board, "case {i} failed!");
            });
    }
}
