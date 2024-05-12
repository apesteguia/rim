use ncurses::*;

#[derive(Debug)]
pub struct Explorer {
    pub path: String,
    pub w: i32,
    pub h: i32,
    pub win: WINDOW,
    pub x: i32,
    pub y: i32,
    pub idx_x: i32,
    pub idx_y: i32,
    pub start: i32,
    pub end: i32,
}
