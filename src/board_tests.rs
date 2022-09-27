#[cfg(test)]
pub mod board_win_tests {
    use crate::{Board, Field, FieldState};

    const O: Field = Field {
        state: FieldState::O,
    };
    const X: Field = Field {
        state: FieldState::X,
    };
    const EMPTY: Field = Field {
        state: FieldState::EMPTY,
    };
    #[test]
    fn for_empty_board_no_winner() {
        let board = Board {
            fields: vec![
                EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY,
            ],
        };
        assert_eq!(board.check_win(), FieldState::EMPTY);
    }
    #[test]
    fn for_board_with_one_x_wins_x() {
        let board = Board {
            fields: vec![X, X, X, X, X, X, X, X, X],
        };
        assert_eq!(board.check_win(), FieldState::X);
    }
    #[test]
    fn for_board_with_one_o_wins_o() {
        let board = Board {
            fields: vec![O, O, O, O, O, O, O, O, O],
        };
        assert_eq!(board.check_win(), FieldState::O);
    }
    #[test]
    fn for_board_with_one_o_and_a_line_of_x_wins_x() {
        let board = Board {
            fields: vec![O, X, EMPTY, EMPTY, X, EMPTY, EMPTY, X, EMPTY],
        };
        assert_eq!(board.check_win(), FieldState::X);
    }
}
