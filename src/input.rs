
/// Reprensent a logical input to player's action
pub enum MotionState {
    Up,
    Down,
    Left,
    Right,
    Jump,
    LeftClick,
}

/// Collect the current player's action status
pub struct PlayerInputStatus {
    left_click: bool,
    click_time: f32,

    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    jump: bool,
}

impl PlayerInputStatus {
    pub fn new() -> PlayerInputStatus {
        Self { 
            left_click: false,
            click_time: 0.,
            forward: false,
            backward: false,
            left: false,
            right: false,
            jump: false,
        }
    }

    ///Change the player's action status from a logical input
    pub fn set_input(&mut self, element: MotionState ,pressed: bool) {
        match element {
            MotionState::Up => self.forward = pressed,
            MotionState::Down => self.backward = pressed,
            MotionState::Right => self.right = pressed,
            MotionState::Left => self.left = pressed,
            MotionState::Jump => self.jump = pressed,
            MotionState::LeftClick => {
                self.left_click = pressed;
                self.click_time = 0.;
            },
            _ => ()
        }
    }

    pub fn left_click(&self) -> bool {
        self.left_click
    }

    /// Return the time the left button have been clicked
    /// If left click is not pressed return 0.
    pub fn click_time(&self) -> f32 {
        if self.left_click {
            self.click_time
        } else {
            0.
        }
    }
    
    pub fn add_click_time(&mut self, click_time: f32) {
        self.click_time += click_time;
    }

    pub fn reset_click_time(&mut self) {
        self.click_time = 0.;
    }
    
    pub fn forward(&self) -> bool {
        self.forward
    }
    
    pub fn backward(&self) -> bool {
        self.backward
    }
    
    pub fn left(&self) -> bool {
        self.left
    }
    
    pub fn right(&self) -> bool {
        self.right
    }
    
    pub fn jump(&self) -> bool {
        self.jump
    }
}


#[cfg(test)]
mod tests {
    use crate::input::PlayerInputStatus;
    use crate::input::MotionState;


    #[test]
    fn test_set_input() {

        let mut input = PlayerInputStatus::new();

        input.set_input(MotionState::Left, true);
        assert_eq!(input.left(), true);
        assert_eq!(input.right(), false);

        input.set_input(MotionState::Right, true);
        assert_eq!(input.left(), true);
        assert_eq!(input.right(), true);

        input.set_input(MotionState::Left, false);
        assert_eq!(input.left(), false);
        assert_eq!(input.right(), true);
    }

    #[test]
    fn test_set_all_input() {

        let mut input = PlayerInputStatus::new();

        input.set_input(MotionState::Left, true);
        input.set_input(MotionState::Right, true);
        input.set_input(MotionState::Up, true);
        input.set_input(MotionState::Down, true);
        input.set_input(MotionState::Jump, true);
        input.set_input(MotionState::LeftClick, true);
        assert_eq!(input.left(), true);
        assert_eq!(input.right(), true);
        assert_eq!(input.forward(), true);
        assert_eq!(input.backward(), true);
        assert_eq!(input.jump(), true);
        assert_eq!(input.left_click(), true);
    }

    #[test]
    fn test_click_time() {

        let mut input = PlayerInputStatus::new();

        input.add_click_time(0.5);
        assert_eq!(input.click_time(), 0.);

        input.set_input(MotionState::LeftClick, true);
        assert_eq!(input.click_time(), 0.);

        input.add_click_time(0.5);
        assert_eq!(input.click_time(), 0.5);

        input.reset_click_time();
        assert_eq!(input.click_time(), 0.);

        input.set_input(MotionState::LeftClick, true);
        input.add_click_time(0.5);
        input.set_input(MotionState::LeftClick, false);
        assert_eq!(input.click_time(), 0.);
    }
}