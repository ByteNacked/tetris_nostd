use super::CType;
use super::Figure;
use super::{BOARD_H, BOARD_W, SYMBOL_EMPTY, SYMBOL_OCCUPIED};

#[derive(Clone, Copy)]
pub struct Cell {
    pub state: CType,
    pub dirty: bool,
}

pub struct Board {
    pub b: [[Cell; BOARD_W]; BOARD_H],
    w: usize,
    h: usize,
}

impl Board {
    pub fn new() -> Board {
        Board {
            b: [[Cell {
                state: CType::Empty,
                dirty: false,
            }; BOARD_W]; BOARD_H],
            w: BOARD_W,
            h: BOARD_H,
        }
    }

    pub fn is_collide_side(&self, f: &Figure) -> bool {
        if f.x < 0 || f.x as usize + f.w > BOARD_W {
            return true;
        }

        self.is_collide(f)
    }

    pub fn is_collide_bottom(&self, f: &Figure) -> bool {
        if f.y as usize + f.h == BOARD_H + 1 {
            return true;
        }

        self.is_collide(f)
    }

    pub fn place(&mut self, f: &Figure) {
        for y in 0..f.h {
            for x in 0..f.w {
                if f.shape[y][x] == SYMBOL_OCCUPIED {
                    self.change_cell(f.x as usize + x, f.y as usize + y, f.style);
                }
            }
        }
    }

    pub fn unplace(&mut self, f: &Figure) {
        for y in 0..f.h {
            for x in 0..f.w {
                if f.shape[y][x] == SYMBOL_OCCUPIED {
                    self.change_cell(f.x as usize + x, f.y as usize + y, CType::Empty);
                }
            }
        }
    }

    const MAX_LINES_REMOVED : usize = 5;
    pub fn remove_lines(&mut self) {
        let mut lines_to_remove : [usize; Self::MAX_LINES_REMOVED] = [0; Self::MAX_LINES_REMOVED];
        let mut lines_sz : usize = 0;

        for y in 0 .. self.h {
            for x in 0 .. self.w {
                if self.b[y][x].state == CType::Empty {
                    break;
                }
                if x == self.w - 1 {
                    lines_to_remove[lines_sz] = y;
                    lines_sz += 1;
                }
            }
        }

        if lines_sz != 0 {
            self.splice_down_board(&mut lines_to_remove, lines_sz);
        }

    }

    fn splice_down_board(&mut self, list : &mut [usize; Self::MAX_LINES_REMOVED], list_sz : usize) {
        for i in 0 .. list_sz {
            self.move_line(list[i], list[i] - i);
        }
    }

    fn move_line(&mut self, dst : usize, src : usize) {
        for x in 0 .. self.w {
            self.b[src][x].dirty = true;
            self.b[dst][x] = self.b[src][x];
            self.b[src][x].state = CType::Empty;
        }
    }

    fn change_cell(&mut self, x: usize, y: usize, c_type: CType) {
        self.b[y][x].state = c_type;
        self.b[y][x].dirty = true;
    }

    fn is_collide(&self, f: &Figure) -> bool {
        for y in 0..f.h {
            for x in 0..f.w {
                let o_x = x + f.x as usize;
                let o_y = y + f.y as usize;
                if self.b[o_y][o_x].state != CType::Empty && f.shape[y][x] == SYMBOL_OCCUPIED {
                    return true;
                }
            }
        }

        false
    }
}
