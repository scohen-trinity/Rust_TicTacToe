use std::io;
use crate::grid::Grid;

mod grid;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Marker {
    X,
    O,
    B,
}

#[derive(Debug, PartialEq)]
enum Player {
    One,
    Two,
}

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

        display_grid(&grid);

        playing = check_grid(&grid);

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

fn display_grid(grid: &Grid) {
    println!("{:?} | {:?} | {:?} ", &grid.positions[0], &grid.positions[1], &grid.positions[2]);
    println!("----------");
    println!("{:?} | {:?} | {:?} ", &grid.positions[3], &grid.positions[4], &grid.positions[5]);
    println!("----------");
    println!("{:?} | {:?} | {:?} ", &grid.positions[6], &grid.positions[7], &grid.positions[8]);
    println!("");
}

fn check_grid(grid: &Grid) -> bool {
    let winning_combos: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for element in &winning_combos {
        if &grid.positions[element[0]] != &Marker::B && &grid.positions[element[0]] == &grid.positions[element[1]] && &grid.positions[element[1]] == &grid.positions[element[2]] {
            println!("{:?} won!", &grid.positions[element[0]]);
            return false;
        }
    }

    true
}