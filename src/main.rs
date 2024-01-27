use std::io;
use tic_tac_toe::{Grid, Player};
use std::num::ParseIntError;
fn main() {
    let mut player = Player::One;
    let mut playing = true;
    let mut user_move = String::new();
    let mut turns: u8 = 1;
    let mut squares: Result<usize, ParseIntError> = Ok(9);
    let mut grid = Grid::empty_grid();

    while playing == true {
        println!("Enter a square number you want to move to: ");
        
        io::stdin().read_line(&mut user_move).unwrap();

        squares = user_move.trim().parse();

        match squares {
            Ok(val) => grid.update_grid(&player, val),
            Err(err) => panic!("Did not enter a valid number, error: {err}"),
        }


        grid.display_grid();

        playing = grid.check_grid();

        user_move = String::from("");

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