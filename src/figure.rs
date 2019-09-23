use super::CType;
use super::BOARD_W;

pub const SYMBOL_EMPTY: u8 = b' ';
pub const SYMBOL_OCCUPIED: u8 = b'*';
const F_SIZE : usize = 4;

#[rustfmt::skip]
const T : [[u8; F_SIZE]; F_SIZE] = [
    [b'*', b'*', b'*', b' ',],
    [b' ', b'*', b' ', b' ',],
    [b' ', b' ', b' ', b' ',],
    [b' ', b' ', b' ', b' ',],
];

#[rustfmt::skip]
const DOT : [[u8; F_SIZE]; F_SIZE] = [
    [b'*', b'*', b' ', b' ',],
    [b'*', b'*', b' ', b' ',],
    [b' ', b' ', b' ', b' ',],
    [b' ', b' ', b' ', b' ',],
];

#[rustfmt::skip]
const I : [[u8; F_SIZE]; F_SIZE] = [
    [b'*', b' ', b' ', b' ',],
    [b'*', b' ', b' ', b' ',],
    [b'*', b' ', b' ', b' ',],
    [b'*', b' ', b' ', b' ',],
];

const FIGURE_LIST: [[[u8; F_SIZE]; F_SIZE]; 2] = [T, DOT];

#[derive(Clone, Copy)]
pub struct Figure {
    pub w: usize,
    pub h: usize,
    pub x: i32,
    pub y: i32,
    pub shape: [[u8; F_SIZE]; F_SIZE],
    pub style: CType,
}

impl Figure {
    pub fn new(rand: u32) -> Self {
        let style = Self::generate_style(rand);

        match rand % 3 {
            0 => Figure {
                    w: 2,
                    h: 2,
                    x: (BOARD_W / 2 - 1) as i32,
                    y: 0,
                    shape: DOT,
                    style,
                },
            1 => Figure {
                    w: 1,
                    h: 4,
                    x: (BOARD_W / 2 - 1) as i32,
                    y: 0,
                    shape: I,
                    style,
                },
            2 => Figure {
                    w: 3,
                    h: 2,
                    x: (BOARD_W / 2 - 1) as i32,
                    y: 0,
                    shape: T,
                    style,
                },
            _ => unreachable!(),
        }

    }

    pub fn rotate(&mut self) {
        //for y in F_SIZE {
        //    for x in F_SIZE {
        //        
        //    }
        //}
    }

    fn generate_style(seed: u32) -> CType {
        CType::from_u32(seed)
    }
}
