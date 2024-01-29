use std::{io, usize};
use tic_tac_toe::{Grid, Player};
use std::num::ParseIntError;
fn main() {
    let mut player: Player = Player::One;
    let mut playing: bool = true;
    let mut turns: u8 = 1;
    let mut grid: Grid = Grid::empty_grid();

    while playing == true {
        
        let square: Result<usize, ParseIntError> = prompt_user();

        match square {
            Ok(val) => grid.update_grid(&player, val),
            Err(err) => panic!("Did not enter a valid number, error: {err}"),
        }

        grid.display_grid();

        playing = grid.check_grid();

        if player == Player::One {
            player = Player::Two;
        } else {
            player = Player::One;
        }

        turns += 1;

        if turns > 9 {
            playing = false;
            println!("It's a tie!")
        }
    }   
}

fn prompt_user() -> Result<usize, ParseIntError> {
    let mut user_move = String::new();

    println!("Enter a square number you want to move to: ");
        
    io::stdin().read_line(&mut user_move).unwrap();

    user_move.trim().parse()
}