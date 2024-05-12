use ncurses::*;

const START_X: i32 = 1;
const START_Y: i32 = 5;

#[derive(Debug)]
pub struct Explorer {
    pub path: String,
    pub dirs: Vec<String>,
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

impl Explorer {
    pub fn new(path: impl Into<String>) -> Self {
        init_color(COLOR_BLACK as i16, 40, 40, 40);
        init_color(COLOR_BLUE as i16, 40, 40, 1000);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);
        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());
        let end = h - 5;

        let win = newwin(h, w, 0, 0);

        Self {
            path: path.into(),
            dirs: Vec::new(),
            w,
            h,
            win,
            x: START_X,
            y: START_Y,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end,
        }
    }

    pub fn get_dirs(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
