use std::io;
use tic_tac_toe::{Grid, Player};
fn main() {
    let mut player = Player::One;
    let mut playing = true;
    let mut user_move = String::new();
    let mut turns: u8 = 1;

    let mut grid = Grid::empty_grid();

    while playing == true {
        println!("Enter a square number you want to move to: ");
        
        io::stdin().read_line(&mut user_move).expect("Failed to read line");

        grid.update_grid(&player, &user_move);

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