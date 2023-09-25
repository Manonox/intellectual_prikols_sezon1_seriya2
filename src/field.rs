use std::hash::{Hash, Hasher};
use std::fmt;
use raylib::prelude::*;

#[allow(unused)]

#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[allow(unused)]
impl Move {
    pub fn iter() -> impl Iterator<Item = Move> {
        [Move::Up, Move::Down, Move::Left, Move::Right].iter().copied()
    }

    pub fn inverse(&self) -> Move {
        match self {
            Move::Up => Move::Down,
            Move::Down => Move::Up,
            Move::Left => Move::Right,
            Move::Right => Move::Left,
        }
    }
}


impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Move::Up => "^",
            Move::Down => "v",
            Move::Left => "<",
            Move::Right => ">",
        })
    }
}


#[derive(Clone, Copy)]
pub struct Field {
    data: u64,
    empty_index: u8
}


#[allow(unused)]
impl Field {
    pub fn new() -> Field {
        Field {
            data: 0x123456789ABCDEF0,
            empty_index: 15,
        }
    }

    pub fn from(state: u64) -> Result<Field, ()> {
        let mut empty_index_opt = None;
        for i in 0..16 {
            if (state >> 4 * (15 - i)) & 0xF == 0 { empty_index_opt = Some(i); break; }
        }

        match empty_index_opt {
            Some(empty_index) => { Ok(Field {
                data: state,
                empty_index: empty_index,
            }) },

            None => { Err(()) }
        }
    }
    

    pub fn from_string<S: AsRef<str>>(s: S) -> Result<Field, ()> {
        let mut f = Field::new();
        f.set_string(s)
    }
    

    pub fn set_string<S: AsRef<str>>(&mut self, s: S) -> Result<Field, ()> {
        let bytes = s.as_ref().as_bytes();
        if bytes.len() != 16 { return Err(()); }
        let mut found = [false; 16];
        let mut newdata = 0x0;
        for i in 0_u8..16_u8 {
            let mut value = bytes[i as usize];
            if value == b'0' {
                self.empty_index = i;
            }

            if value >= b'0' && value <= b'9' {
                value -= b'0';
            } else if value >= b'A' && value <= b'F' {
                value -= b'A';
                value += 10;
            } else {
                return Err(());
            }

            newdata |= (value as u64) << 4 * (15 - i);
            found[value as usize] = true;
        }

        let success = (0..16).all(|i| {found[i]});
        if !success { return Err(()); }

        self.data = newdata;
        Ok(*self)
    }


    pub fn get_string(&self) -> String {
        let mut s = String::with_capacity(16);
        (0..15).for_each(|i| {
            let v = (self.data >> 4 * (15 - i)) as u8;
            s.push((if v < 10 { b'0' + v } else { b'A' + (v - 10) }) as char);
        });
        s
    }


    pub fn get_linear<I: Into<u8>>(&self, i: I) -> u8 {
        ((self.data >> 4 * (15 - i.into())) & 0xF) as u8
    }


    pub fn get_empty_index(&self) -> u8 {
        self.empty_index
    }


    pub fn is_solvable(&self) -> bool {
        let mut sum: u8 = 0;
        for i in 0_u8..16_u8 {
            let value_i = self.get_linear(i);
            if value_i == 0 {
                sum += i / 4;
                continue;
            }
            for j in 0_u8..(i+1) {
                let value_j = self.get_linear(j);
                if value_j == 0 { continue }
                if value_j < value_i { sum += 1 }
            }
        }

        return sum % 2 == 0;
    }


    pub fn is_solved(&self) -> bool {
        self.data == 0x123456789ABCDEF0
    }


    fn swap<I: Into<u8>>(&mut self, i1: I, i2: I) {
        let shift1 = 4 * (15 - i1.into());
        let shift2 = 4 * (15 - i2.into());
        let mask1 = 0xF << shift1;
        let mask2 = 0xF << shift2;
        let bits1 = self.data & mask1;
        let bits2 = self.data & mask2;
        self.data &= !mask1;
        self.data &= !mask2;
        self.data |= (bits1 >> shift1) << shift2;
        self.data |= (bits2 >> shift2) << shift1;
    }


    fn swap_with_empty(&mut self, move_index: u8) -> bool {
        self.swap(self.empty_index, move_index);
        self.empty_index = move_index;
        true
    }


    pub fn is_valid_move(&self, m: Move) -> bool {
        match m {
            Move::Up => { self.empty_index > 3 }
            Move::Down => { self.empty_index < 12 }
            Move::Left => { self.empty_index % 4 > 0 }
            Move::Right => { self.empty_index % 4 < 3 }
        }
    }


    pub fn up(&mut self) -> bool {
        if !self.is_valid_move(Move::Up) { return false }
        self.swap_with_empty(self.empty_index - 4)
    }

    pub fn down(&mut self) -> bool {
        if !self.is_valid_move(Move::Down) { return false }
        self.swap_with_empty(self.empty_index + 4)
    }

    pub fn left(&mut self) -> bool {
        if !self.is_valid_move(Move::Left) { return false }
        self.swap_with_empty(self.empty_index - 1)
    }

    pub fn right(&mut self) -> bool {
        if !self.is_valid_move(Move::Right) { return false }
        self.swap_with_empty(self.empty_index + 1)
    }


    pub fn make_move(&mut self, m: Move) -> bool {
        match m {
            Move::Up    => { self.up()      },
            Move::Down  => { self.down()    },
            Move::Left  => { self.left()    },
            Move::Right => { self.right()   },
        }
    }


    pub fn unique_id(&self) -> u64 {
        return self.data;
    }


    pub fn display(&self) {
        (0..13).step_by(4).for_each(|i| {
            println!("{:>4} {:>4} {:>4} {:>4}", self.get_linear(i), self.get_linear(i + 1), self.get_linear(i + 2), self.get_linear(i + 3));
        });
    }
}


impl Default for Field {
    fn default() -> Self {
        Field::new()
    }
}


impl PartialEq for Field{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}


impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.data);
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
