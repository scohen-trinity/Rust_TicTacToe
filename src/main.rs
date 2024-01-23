use std::io;

#[derive(Debug)]
struct Grid {
    positions: [Marker; 9],
}

impl Grid {
    fn empty_grid() -> Self {
        Grid {
            positions: [Marker::B; 9],
        }
    }

    fn update_grid(&mut self, player: &Player, user_move: &str) {
        println!("spot moved to {}", user_move);
        let square: usize = user_move.trim().parse().expect("Could not parse into int");
        match player {
            Player::One => {
                if self.positions[square] == Marker::B {
                    self.positions[square] = Marker::X
                }
            },
            Player::Two => {
                if self.positions[square] == Marker::B {
                    self.positions[square] = Marker::O
                }
            },
        }
    }
}

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
}

fn check_grid(grid: &Grid) -> bool {
    if &grid.positions[0] != &Marker::B && &grid.positions[0] == &grid.positions[1] && &grid.positions[1] == &grid.positions[2] {
        println!("{:?} won!", &grid.positions[0]);
        return false;
    } else if &grid.positions[3] != &Marker::B && &grid.positions[3] == &grid.positions[4] && &grid.positions[4] == &grid.positions[5] {
        println!("{:?} won!", &grid.positions[3]);
        return false;
    } else if &grid.positions[6] != &Marker::B && &grid.positions[6] == &grid.positions[7] && &grid.positions[7] == &grid.positions[8] {
        println!("{:?} won!", &grid.positions[6]);
        return false;
    } else if &grid.positions[0] != &Marker::B && &grid.positions[0] == &grid.positions[3] && &grid.positions[3] == &grid.positions[6] {
        println!("{:?} won!", &grid.positions[0]);
        return false;
    } else if &grid.positions[1] != &Marker::B && &grid.positions[1] == &grid.positions[4] && &grid.positions[4] == &grid.positions[7] {
        println!("{:?} won!", &grid.positions[1]);
        return false;
    } else if &grid.positions[2] != &Marker::B && &grid.positions[2] == &grid.positions[5] && &grid.positions[5] == &grid.positions[8] {
        println!("{:?} won!", &grid.positions[2]);
        return false;
    } else if &grid.positions[0] != &Marker::B && &grid.positions[0] == &grid.positions[4] && &grid.positions[4] == &grid.positions[8] {
        println!("{:?} won!", &grid.positions[0]);
        return false;
    } else if &grid.positions[2] != &Marker::B && &grid.positions[2] == &grid.positions[4] && &grid.positions[4] == &grid.positions[6] {
        println!("{:?} won!", &grid.positions[2]);
        return false;
    } 

    true
}