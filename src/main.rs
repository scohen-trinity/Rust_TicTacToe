use std::{io, usize};
use tic_tac_toe::{Grid, Player};
use std::num::ParseIntError;
fn main() {
    let mut player: Player = Player::One;
    let mut playing: bool = true;
    let mut turns: u8 = 1;
    let mut grid: Grid = Grid::empty_grid();

    while playing == true {
        
        let user_move: usize = get_user_move();

        grid.update_grid(&player, user_move);

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

fn get_user_move() -> usize {
    let mut user_move: String = String::new();

    println!("Enter a square number you want to move to: ");
        
    io::stdin().read_line(&mut user_move).unwrap();

    let user_move: Result<usize, ParseIntError> = user_move.trim().parse();

    match user_move {
        Ok(val) => { return val; },
        Err(err) => {
            println!("Did not enter a valid square, try again!: {err}");
            return get_user_move();
        },
    }
}