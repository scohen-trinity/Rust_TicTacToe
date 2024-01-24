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

    pub fn update_grid(&mut self, player: &Player, user_move: &str) {
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