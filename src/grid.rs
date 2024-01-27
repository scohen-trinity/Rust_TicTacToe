use crate::Player;
use crate::Marker;

#[derive(Debug)]
pub struct Grid {
    pub positions: [Marker; 9],
}

impl Grid {
    pub fn empty_grid() -> Self {
        Grid {
            positions: [Marker::B; 9],
        }
    }

    pub fn update_grid(&mut self, player: &Player, user_move: usize) {
        println!("spot moved to {}", user_move);
        match player {
            Player::One => {
                if self.positions[user_move] == Marker::B {
                    self.positions[user_move] = Marker::X
                }
            },
            Player::Two => {
                if self.positions[user_move] == Marker::B {
                    self.positions[user_move] = Marker::O
                }
            },
        }
    }

    pub fn check_grid(&self) -> bool {
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
            if &self.positions[element[0]] != &Marker::B && &self.positions[element[0]] == &self.positions[element[1]] && &self.positions[element[1]] == &self.positions[element[2]] {
                println!("{:?} won!", &self.positions[element[0]]);
                return false;
            }
        }
    
        true
    }

    pub fn display_grid(&self) {
        println!("{:?} | {:?} | {:?} ", &self.positions[0], &self.positions[1], &self.positions[2]);
        println!("----------");
        println!("{:?} | {:?} | {:?} ", &self.positions[3], &self.positions[4], &self.positions[5]);
        println!("----------");
        println!("{:?} | {:?} | {:?} ", &self.positions[6], &self.positions[7], &self.positions[8]);
        println!("");
    }
}