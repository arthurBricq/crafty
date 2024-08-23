const N_ROWS: f32 = 8.;
const N_COLS: f32 = 16.;

/// A character that can be rendered with our OpenGL pipeline
#[derive(Clone,Copy)]
pub enum GLChar {
    A,B,C,D,E,F,G,H,I,J,K,L,M,N,O,P,Q,R,S,T,U,V,W,X,Y,Z,
    DOT,DOUBLEPOINT,COMMA,MINUS,
    ZERO,ONE,TWO,THREE,FOUR,FIVE,SIX,SEVEN,EIGHT,NINE
}

impl GLChar {
    pub fn from_char(c: char) -> Self {
        match c {
            'a' => GLChar::A,
            'b' => GLChar::B,
            'c' => GLChar::C,
            'd' => GLChar::D,
            'e' => GLChar::E,
            'f' => GLChar::F,
            'g' => GLChar::G,
            'h' => GLChar::H,
            'i' => GLChar::I,
            'j' => GLChar::J,
            'k' => GLChar::K,
            'l' => GLChar::L,
            'm' => GLChar::M,
            'n' => GLChar::N,
            'o' => GLChar::O,
            'p' => GLChar::P,
            'q' => GLChar::Q,
            'r' => GLChar::R,
            's' => GLChar::S,
            't' => GLChar::T,
            'u' => GLChar::U,
            'v' => GLChar::V,
            'w' => GLChar::W,
            'x' => GLChar::X,
            'y' => GLChar::Y,
            'z' => GLChar::Z,
            '.' => GLChar::DOT,
            ':' => GLChar::DOUBLEPOINT,
            ',' => GLChar::COMMA,
            '-' => GLChar::MINUS,
            '0' => GLChar::ZERO,
            '1' => GLChar::ONE,
            '2' => GLChar::TWO,
            '3' => GLChar::THREE,
            '4' => GLChar::FOUR,
            '5' => GLChar::FIVE,
            '6' => GLChar::SIX,
            '7' => GLChar::SEVEN,
            '8' => GLChar::EIGHT,
            '9' => GLChar::NINE,
            _ => panic!("Character is not supported: {c}")
        }
    }

    /// Returns the index of the bottom-left corner in the font atlas
    pub fn get_index(&self) -> [f32;2] {
        match self {
            GLChar::A => [1. / N_COLS, 3. / N_ROWS],
            GLChar::B => [2. / N_COLS, 3. / N_ROWS],
            GLChar::C => [3. / N_COLS, 3. / N_ROWS],
            GLChar::D => [4. / N_COLS, 3. / N_ROWS],
            GLChar::E => [5. / N_COLS, 3. / N_ROWS],
            GLChar::F => [6. / N_COLS, 3. / N_ROWS],
            GLChar::G => [7. / N_COLS, 3. / N_ROWS],
            GLChar::H => [8. / N_COLS, 3. / N_ROWS],
            GLChar::I => [9. / N_COLS, 3. / N_ROWS],
            GLChar::J => [10. / N_COLS, 3. / N_ROWS],
            GLChar::K => [11. / N_COLS, 3. / N_ROWS],
            GLChar::L => [12. / N_COLS, 3. / N_ROWS],
            GLChar::M => [13. / N_COLS, 3. / N_ROWS],
            GLChar::N => [14. / N_COLS, 3. / N_ROWS],
            GLChar::O => [15. / N_COLS, 3. / N_ROWS],
            GLChar::P => [0. / N_COLS, 2. / N_ROWS],
            GLChar::Q => [1. / N_COLS, 2. / N_ROWS],
            GLChar::R => [2. / N_COLS, 2. / N_ROWS],
            GLChar::S => [3. / N_COLS, 2. / N_ROWS],
            GLChar::T => [4. / N_COLS, 2. / N_ROWS],
            GLChar::U => [5. / N_COLS, 2. / N_ROWS],
            GLChar::V => [6. / N_COLS, 2. / N_ROWS],
            GLChar::W => [7. / N_COLS, 2. / N_ROWS],
            GLChar::X => [8. / N_COLS, 2. / N_ROWS],
            GLChar::Y => [9. / N_COLS, 2. / N_ROWS],
            GLChar::Z => [10. / N_COLS, 2. / N_ROWS],
            GLChar::COMMA => [12. / N_COLS, 5. / N_ROWS],
            GLChar::DOT => [14. / N_COLS, 5. / N_ROWS],
            GLChar::DOUBLEPOINT => [10. / N_COLS, 4. / N_ROWS],
            GLChar::MINUS => [13. / N_COLS, 5. / N_ROWS],
            GLChar::ZERO => [0. / N_COLS, 4. / N_ROWS],
            GLChar::ONE => [1. / N_COLS, 4. / N_ROWS],
            GLChar::TWO => [2. / N_COLS, 4. / N_ROWS],
            GLChar::THREE => [3. / N_COLS, 4. / N_ROWS],
            GLChar::FOUR => [4. / N_COLS, 4. / N_ROWS],
            GLChar::FIVE => [5. / N_COLS, 4. / N_ROWS],
            GLChar::SIX => [6. / N_COLS, 4. / N_ROWS],
            GLChar::SEVEN => [7. / N_COLS, 4. / N_ROWS],
            GLChar::EIGHT => [8. / N_COLS, 4. / N_ROWS],
            GLChar::NINE => [9. / N_COLS, 4. / N_ROWS],
        }
    }

    /// Returns the dimensions of each font in the font atlas
    pub fn get_offset() -> [f32;2] {
        [1. / N_COLS * 0.7, 1. / N_ROWS]
    }
}