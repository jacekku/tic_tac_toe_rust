pub mod board_tests;

use std::{io, num::ParseIntError};

#[derive(PartialEq, Debug)]
enum FieldState {
    EMPTY,
    X,
    O,
}
pub struct Field {
    state: FieldState,
}

impl Field {
    fn new() -> Field {
        Field {
            state: FieldState::EMPTY,
        }
    }

    fn set(&mut self, ns: FieldState) {
        self.state = ns;
    }
}
struct Board {
    pub fields: Vec<Field>,
}

struct GameState {
    current_player: FieldState,
}

impl GameState {
    fn player_str(&self) -> &str {
        match self.current_player {
            FieldState::O => "O",
            FieldState::X => "X",
            _ => "invalid state",
        }
    }
    fn player_token(&self) -> FieldState {
        match self.current_player {
            FieldState::X => FieldState::X,
            FieldState::O => FieldState::O,
            FieldState::EMPTY => FieldState::EMPTY,
        }
    }

    fn flip_player(&mut self) {
        match self.current_player {
            FieldState::X => self.current_player = FieldState::O,
            FieldState::O => self.current_player = FieldState::X,
            FieldState::EMPTY => self.current_player = FieldState::X,
        }
    }
}

impl Board {
    fn new(size: usize) -> Board {
        let mut new_fields: Vec<Field> = Vec::new();
        for _ in 0..size {
            new_fields.push(Field::new());
        }

        Board { fields: new_fields }
    }

    fn can_place(&mut self, &index: &usize) -> bool {
        if index >= self.fields.len() {
            return false;
        }
        return self.fields[index].state == FieldState::EMPTY;
    }

    fn print(&mut self, snap: usize) -> String {
        let mut output = String::new();
        for i in 0..9 {
            if i % snap == 0 {
                output.push_str("\n");
            }
            output.push_str(match self.fields[i].state {
                FieldState::EMPTY => " ",
                FieldState::O => "O",
                FieldState::X => "X",
            });
        }
        output
    }

    pub fn check_win(&self) -> FieldState {
        let winning_indices = vec![
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];
        for indices in winning_indices {
            if self.fields[indices[0]].state == self.fields[indices[1]].state
                && self.fields[indices[0]].state == self.fields[indices[2]].state
            {
                match self.fields[indices[0]].state {
                    FieldState::EMPTY => continue,
                    FieldState::O => return FieldState::O,
                    FieldState::X => return FieldState::X,
                }
            }
        }
        return FieldState::EMPTY;
    }
}

fn get_parsed_user_input(prompt: String) -> Result<usize, ParseIntError> {
    println!("{}", prompt);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    Ok(input.trim().parse::<usize>()?)
}

fn main() {
    const BOARD_SIZE: usize = 9;
    let mut board = Board::new(BOARD_SIZE);
    let mut game_state = GameState {
        current_player: FieldState::X,
    };
    loop {
        let mut index: Result<usize, ParseIntError> = Ok(10);
        println!("board:");
        print!("___");
        println!("{}", board.print(3));
        println!("---");
        println!("now moves {}", game_state.player_str());

        loop {
            valid_index(&mut index);
            match index {
                Ok(number) => {
                    if board.can_place(&number) {
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
        match index {
            Ok(number) => {
                println!("you picked: {}", number);
                board.fields[number].set(game_state.player_token());
                game_state.flip_player();
            }
            Err(error) => panic!("after all that something went wrong {}", error),
        }

        if board.check_win() != FieldState::EMPTY {
            break;
        }
    }
    println!("game ended");
    game_state.flip_player();
    println!("{} wins!", game_state.player_str());
}

fn valid_index(index: &mut Result<usize, ParseIntError>) {
    loop {
        *index = get_parsed_user_input("pick a value ".to_owned());
        match *index {
            Ok(_) => break,
            Err(_) => continue,
        }
    }
}
