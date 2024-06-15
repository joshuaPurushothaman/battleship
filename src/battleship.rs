use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy)]
pub struct Ship {
    pub start: Vec2i,
    pub end: Vec2i,
}

#[derive(Clone, Copy)]
pub enum GridCell {
    Blank,
    White,
    Red,
}

impl Display for GridCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ░▒▓

        const RED: &str = "\x1b[31m";
        const RESET: &str = "\x1b[0m";

        match self {
            GridCell::Blank => write!(f, "░"),
            GridCell::White => write!(f, "▓"),
            GridCell::Red => write!(f, "{RED}░{RESET}"),
        }
    }
}

pub struct Board {
    ships: [Ship; 5],
    cells: [[GridCell; 10]; 10],
}

impl Board {
    pub fn new(ships: [Ship; 5]) -> Self {
        let cells = [[GridCell::Blank; 10]; 10];

        Self { ships, cells }
    }

    pub fn call_shot(coord: Vec2i) {
        let _ = coord;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for Windows devs: look for "Character Map" preinstalled!

        // ─│┌ ┐└ ┘├ ┤┬ ┴ ┼ Single

        // ═║╔ ╗╚ ╝╠ ╣╦ ╩ ╬ Double

        // ╓ ╖╥ ╟ ╢   Double vertical
        // ╙ ╜╨  ╫
        // ╒ ╕╤ ╞  ╡  Double horizontal
        // ╘ ╛╧   ╪

        for row in self.cells {
            for cell in row {
                // TODO: ships... lol
                write!(f, "{cell}")?
            }
        }

        Ok(())
    }
}
