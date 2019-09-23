#![no_std]

mod figure;
use figure::{Figure, SYMBOL_EMPTY, SYMBOL_OCCUPIED};

mod board;
use board::{Board, Cell};

mod control;
pub use control::Control;

mod rand;
use rand::XorShift32 as Rand;

pub const BOARD_W: usize = 6;//20;
pub const BOARD_H: usize = 30;
const TICKS_TO_STEP: usize = 30;
const BOTTOM_LINE: usize = BOARD_H - 1;

pub struct Game {
    b: Board,
    tick: usize,
    f: Figure,
    score: usize,
    c: Control,
    rand: Rand,
}

impl Game {
    pub fn new(seed: u64) -> Self {
        let mut rand = Rand::new(seed as u32);
        let mut b = Board::new();
        let f = Figure::new(rand.next());
        b.place(&f);

        Game {
            b,
            f,
            score: 0,
            c: Control::default(),
            tick: 0,
            rand,
        }
    }

    pub fn control(&mut self, c: &Control) {
        self.c = c.clone();
    }

    pub fn update(&mut self) {
        let mut dx = 0i32;
        let mut dy = 0i32;

        if self.c.fall {
            self.fall_figure();
            self.tick += 1;
            return;
        }

        if self.tick % TICKS_TO_STEP == 0 {
            dy += 1;
            self.b.remove_lines();
            //++println!("Game tick : {}", &self.tick);
        }

        if self.c.left {
            dx -= 1;
        }

        if self.c.right {
            dx += 1;
        }

        if dx != 0 || dy != 0 {
            self.move_figure(dx, dy);
        }

        self.tick += 1;
    }

    pub fn get_draw_cell(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.b.b[y][x]
    }

    fn move_figure(&mut self, xd: i32, yd: i32) {
        if xd == 0 && yd == 0 {
            return;
        }

        self.b.unplace(&self.f);
        self.f.x += xd;
        self.f.y += yd;
        let mut is_col_bot = false;

        if xd != 0 {
            if self.b.is_collide_side(&self.f) {
                self.f.x -= xd;
            }
        };

        if yd != 0 {
            if self.b.is_collide_bottom(&self.f) {
                self.f.y -= yd;
                is_col_bot = true;
            }
        };

        self.b.place(&self.f);
        //Creating new figure coz previous just landed
        if is_col_bot {
            self.new_figure();
            self.b.place(&self.f);
        }
    }

    fn fall_figure(&mut self) {
        self.b.unplace(&self.f);

        self.f.y += 1;
        while !self.b.is_collide_bottom(&self.f) {
            self.f.y += 1;
        }
        self.f.y -= 1;

        self.b.place(&self.f);
        //New figure
        self.new_figure();
        self.b.place(&self.f);
    }

    fn new_figure(&mut self) {
        self.f = Figure::new(self.rand.next());
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CType {
    Empty,
    Red,
    Blue,
    C1,
    C2,
    C3,
    C4,
    MAX_NUM,
}

impl CType {
    fn from_u32(mut n: u32) -> Self {
        n = n % ((Self::MAX_NUM as u32) - 1);
        match n {
            0 => CType::Red,
            1 => CType::Blue,
            2 => CType::C1,
            3 => CType::C2,
            4 => CType::C3,
            5 => CType::C4,
            _ => panic!("Cannot convert n to enum"),
        }
    }
}
