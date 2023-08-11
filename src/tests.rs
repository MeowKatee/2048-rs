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
        let pairs = [
            (
                [
                    [None, None, None, NonZeroU8::new(1)],
                    [None; 4],
                    [None, None, NonZeroU8::new(3), None],
                    [None, None, NonZeroU8::new(3), NonZeroU8::new(1)],
                ],
                Arrow::Down,
                [
                    [None; 4],
                    [None; 4],
                    [None; 4],
                    [None, None, NonZeroU8::new(4), NonZeroU8::new(2)],
                ],
            ),
            (
                [[None; 4], [None; 4], [None; 4], [NonZeroU8::new(4); 4]],
                Arrow::Right,
                [
                    [None; 4],
                    [None; 4],
                    [None; 4],
                    [None, None, NonZeroU8::new(5), NonZeroU8::new(5)],
                ],
            ),
            (
                [
                    [None; 4],
                    [None; 4],
                    [None; 4],
                    [NonZeroU8::new(1), None, None, NonZeroU8::new(1)],
                ],
                Arrow::Left,
                [
                    [None; 4],
                    [None; 4],
                    [None; 4],
                    [NonZeroU8::new(2), None, None, None],
                ],
            ),
        ];
        pairs
            .into_iter()
            .map(|(left, op, right)| (Board::from(left), op, Board::from(right)))
            .for_each(|(mut left, op, right)| {
                left.merge(op);
                assert!(left == right)
            });
    }
}
