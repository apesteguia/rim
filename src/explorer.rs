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
    pub fn new(path: impl Into<String> + Copy) -> Self {
        init_color(COLOR_BLACK as i16, 40, 40, 40);
        init_color(COLOR_BLUE as i16, 40, 40, 1000);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);

        let max_width = getmaxx(stdscr());
        let max_height = getmaxy(stdscr());

        let width_with_margin = max_width as f32 * 0.6;
        let height_with_margin = max_height as f32 * 0.6;

        let x_position = ((max_width as f32 - width_with_margin) / 2.0) as i32;
        let y_position = ((max_height as f32 - height_with_margin) / 2.0) as i32;

        let win = newwin(
            height_with_margin as i32,
            width_with_margin as i32,
            y_position,
            x_position,
        );

        let mut p = path.into();
        if let Ok(current_dir) = std::env::current_dir() {
            let file_path = current_dir.join(p);
            let buf = std::path::Path::new(&file_path);
            if let Some(parent_dir) = buf.parent() {
                p = parent_dir.display().to_string();
            } else {
                p = "NO DATA".to_string();
            }
        }

        Self {
            path: p,
            dirs: Vec::new(),
            w: width_with_margin as i32,
            h: height_with_margin as i32,
            win,
            x: START_X,
            y: START_Y,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end: max_height - 5,
        }
    }

    pub fn display(&self) {
        box_(self.win, 0, 0);
        wattron(self.win, COLOR_PAIR(2) | A_BOLD());
        mvwprintw(self.win, 0, 1, &self.path);
        wattroff(self.win, COLOR_PAIR(2) | A_BOLD());
    }

    pub fn update(&mut self) {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        while ch != 113 {
            match ch {
                _ => {
                    self.x += 0;
                }
            }
            self.display();
            ch = wgetch(self.win);
        }
    }

    pub fn get_dirs(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
