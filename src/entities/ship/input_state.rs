pub enum TurnDirection {
    Left = -1,
    Right = 1,
    None = 0,
}

#[derive(Default)]
pub struct PlayerShipInputState {
    pub turning: TurnDirection,
    pub special: bool,
}

impl Default for TurnDirection {
    fn default() -> Self {
        TurnDirection::None
    }
}
