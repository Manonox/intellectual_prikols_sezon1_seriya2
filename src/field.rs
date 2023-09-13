use raylib::prelude::*;


pub struct Field {
    data: [u8; 16],
    empty_index: usize
}

impl Field {
    pub fn new() -> Field {
        let mut f = Field { data: [0; 16], empty_index: 15 };
        for i in 0..16 {
            f.data[i] = ((i + 1) % 16) as u8;
        }
        f
    }


    pub fn from<S: AsRef<str>>(state: S) -> Field {
        let mut f = Field { data: [0; 16], empty_index: 0 };
        f.set_state(state);
        f
    }
    

    pub fn set_state<S: AsRef<str>>(&mut self, state: S) {
        let bytes = state.as_ref().as_bytes();
        for i in 0..16 {
            let mut value = bytes[i];
            if value == b'0' {
                self.empty_index = i;
            }

            if value >= b'0' && value <= b'9' {
                value -= b'0';
            } else if value >= b'A' && value <= b'F' {
                value -= b'A';
                value += 10;
            }

            self.data[i] = value;
        }
    }


    pub fn get_state(&self) -> String {
        let mut state = String::new();
        for i in 0..15 {
            let c = self.data[i] as char;
            state.replace_range(3..4, String::from(c).as_str());
        }
        state
    }


    fn get_linear<I: Into<usize>>(&self, i: I) -> u8 {
        return self.data[Into::<usize>::into(i)];
    }


    pub fn is_solvable(&self) -> bool {
        let mut sum: u8 = 0;
        for i in 0..16 {
            let value = self.get_linear(i);
            if value == 0 {
                sum = i / 4;
                break;
            }
        }

        for i in 0_u8..16_u8 {
            let value_i = self.get_linear(i);
            if value_i == 0 { continue }
            for j in 0_u8..(i+1) {
                let value_j = self.get_linear(j);
                if value_j == 0 { continue }
                if value_j < value_i { sum += 1 }
            }
        }

        return sum % 2 == 0;
    }


    fn swap<I: Into<usize>>(&mut self, i1: I, i2: I) {
        self.data.swap(Into::<usize>::into(i1), Into::<usize>::into(i2));
    }


    pub fn up(&mut self) -> bool {
        if self.empty_index <= 3 { return false }
        let move_index = self.empty_index - 4;
        self.swap(self.empty_index, move_index);
        self.empty_index = move_index;
        true
    }

    pub fn down(&mut self) -> bool {
        if self.empty_index > 11 { return false }
        let move_index = self.empty_index + 4;
        self.swap(self.empty_index, move_index);
        self.empty_index = move_index;
        true
    }

    pub fn left(&mut self) -> bool {
        if self.empty_index % 4 == 0 { return false }
        let move_index = self.empty_index - 1;
        self.swap(self.empty_index, move_index);
        self.empty_index = move_index;
        true
    }

    pub fn right(&mut self) -> bool {
        if self.empty_index % 4 == 3 { return false }
        let move_index = self.empty_index + 1;
        self.swap(self.empty_index, move_index);
        self.empty_index = move_index;
        true
    }


    pub fn display(&self) {
        for i in (0..13).step_by(4) {
            println!("{:>4} {:>4} {:>4} {:>4}", self.data[i], self.data[i + 1], self.data[i + 2], self.data[i + 3]);
        }
    }
}

pub trait Drawable {
    fn draw(&self, gfx: RaylibDrawHandle<'_>);
}

impl Drawable for Field {
    fn draw(&self, mut gfx: RaylibDrawHandle<'_>) {
        for i in 0_u8..16_u8 {
            let value = self.get_linear(i);
            if value == 0 { continue }

            let text = format!("{}", value);
            let x = 20 + 64 * ((i as i32) % 4);
            let y = 20 + 64 * ((i as i32) / 4);
            gfx.draw_text(text.as_str(), x, y, 32, Color::BLACK);
        }
    }
}
