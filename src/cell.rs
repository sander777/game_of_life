#[derive(Copy, Clone)]
pub enum CellState {
    Alive,
    Dead,
    GoingToDie,
    GoingToBorn,
}

impl CellState {
    pub fn calculate_state(self: &mut Self, neighbors_amount: u8) {
        match self {
            CellState::Dead => {
                if neighbors_amount == 3 {
                    *self = CellState::GoingToBorn;
                }
            }
            CellState::Alive => {
                if neighbors_amount > 3 || neighbors_amount < 2 {
                    *self = CellState::GoingToDie;
                }
            }
            _ => {}
        }
    }

    pub fn process_state(self: &mut Self) {
        match self {
            CellState::GoingToBorn => {
                *self = CellState::Alive;
            }
            CellState::GoingToDie => {
                *self = CellState::Dead;
            }
            _ => {}
        }
    }
}
