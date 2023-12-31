use crate::game::ai::*;

pub enum Player {
    Human,
    Computer(Ai),
}

impl Player {
    pub fn is_human(&self) -> bool {
        self.eq(Player::Human)
    }

    fn eq(&self, other: Self) -> bool {
        match (self, other) {
            (Player::Human, Player::Human) => true,
            (Player::Computer(_), Player::Computer(_)) => true,
            _ => false,
        }
    }
}
