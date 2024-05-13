use crate::constants::obtener_nombre_lenguaje;
use crate::explorer;
use crate::file::{format_permissions, Archivo};
use ncurses::*;

const START_X: i32 = 5; // x=0 in the editor
const START_Y: i32 = 1; // y=0 in the editor

#[derive(Debug)]
pub struct State {
    pub archivo: Archivo,
    pub w: i32,
    pub h: i32,
    pub win: WINDOW,
    pub x: i32,
    pub y: i32,
    pub mode: bool,
    pub idx_x: usize,
    pub idx_y: usize,
    pub start: i32,
    pub end: i32,
    pub explorer: explorer::Explorer,
}

impl State {
    pub fn new(path: impl Into<String> + Copy) -> State {
        initscr();
        noecho();
        keypad(stdscr(), true);
        raw();
        start_color();
        cbreak();
        init_color(COLOR_BLACK as i16, 40, 40, 40);
        init_color(COLOR_BLUE as i16, 40, 40, 1000);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);

        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());
        let end = h - 5;

        let win = newwin(h, w, 0, 0);

        State {
            archivo: Archivo::new(&path.into()),
            w,
            h,
            win,
            x: START_X,
            y: START_Y,
            mode: false,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end,
            explorer: explorer::Explorer::new(&path.into()),
        }
    }

    pub fn display(&self) {
        //wclear(self.win);
        //box_(self.win, 0, 0);

        curs_set(CURSOR_VISIBILITY::CURSOR_VISIBLE);
        wattron(self.win, COLOR_PAIR(2) | A_BOLD());
        mvwprintw(self.win, 0, 1, &self.archivo.path);
        wattroff(self.win, COLOR_PAIR(2) | A_BOLD());

        for (_idx, i) in (self.start..self.end + self.start).enumerate() {
            if i > (self.archivo.buffer.len() - 1) as i32 {
                break;
            }

            let format = if i < 10 {
                format!(
                    "{}     ",
                    self.archivo.buffer[i as usize]
                        .iter()
                        .cloned()
                        .collect::<String>()
                )
            } else if i < 100 {
                format!(
                    "{}    ",
                    self.archivo.buffer[i as usize]
                        .iter()
                        .cloned()
                        .collect::<String>()
                )
            } else if i < 1000 {
                format!(
                    "{}   ",
                    self.archivo.buffer[i as usize]
                        .iter()
                        .cloned()
                        .collect::<String>()
                )
            } else {
                self.archivo.buffer[i as usize]
                    .iter()
                    .cloned()
                    .collect::<String>()
                    .to_string()
            };

            wattron(self.win, COLOR_PAIR(3));
            mvwprintw(self.win, (_idx + 1) as i32, 1, &i.to_string());
            wattroff(self.win, COLOR_PAIR(3));
            mvwprintw(self.win, (_idx + 1) as i32, START_X, &format);
        }

        self.display_bar();
        wrefresh(self.win);
    }

    pub fn display_bar(&self) {
        let metadata = self.archivo.file.metadata().unwrap();
        let per = format_permissions(metadata.permissions(), false);

        let file = self.archivo.path.split('/').last().unwrap();
        let lenguaje = file.split('.').last().unwrap();
        let lang = obtener_nombre_lenguaje(lenguaje).unwrap();

        let format = format!(
            "{:?} {}  {}KB  {}:{}  x:{} y:{} realx:{} realy:{}",
            lang,
            per,
            metadata.len(),
            self.idx_y,
            self.archivo.buffer.len(),
            self.x,
            self.y,
            self.idx_x,
            self.idx_y,
        );

        let x = getmaxx(self.win);
        wattron(self.win, COLOR_PAIR(2) | A_BOLD());
        mvwhline(self.win, self.h - 3, 1, 32, x - 2);
        if !self.mode {
            mvwprintw(self.win, self.h - 3, 2, "NORMAL");
        } else {
            mvwprintw(self.win, self.h - 3, 2, "INSERT");
        }
        mvwprintw(self.win, self.h - 3, 10, &format);
        wattroff(self.win, COLOR_PAIR(2) | A_BOLD());
        wmove(self.win, self.y, self.x);
        wrefresh(self.win);
    }

    pub fn update(&mut self) {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        while ch != 113 {
            self.mode = false;
            match ch {
                100 => {
                    self.handle_delete(&mut ch);
                }
                106 => {
                    self.handle_movment_down();
                }
                107 => {
                    self.handle_movment_up();
                }
                104 => {
                    self.handle_movment_left();
                }
                108 => {
                    self.handle_movment_right();
                }
                98 => {
                    self.handle_start_line();
                }
                32 => {
                    self.explorer.get_files().expect("EXPLORER CANT READ DIRS");
                    self.explorer.display();
                    let a = self.explorer.update();
                    wclear(self.win);
                    match a {
                        Some(str) => {
                            delwin(self.win);
                            *self = State::new(&str);
                            endwin();
                            self.display();
                            self.update();
                        }
                        None => continue,
                    };
                    break;
                }
                9 | 11 => {
                    self.x += 0;
                }
                118 => {
                    self.handle_v();
                }
                KEY_ENTER | 10 | 111 => {
                    self.handle_enter();
                }
                103 => {
                    self.handle_save();
                }
                105 => {
                    self.handle_insert(&mut ch);
                }
                _ => {
                    self.x += 0;
                }
            }
            self.display();
            ch = wgetch(self.win);
        }

        endwin();
    }

    fn handle_v(&mut self) {
        self.idx_x = 0;
        self.x = START_X;
        self.idx_y = 0;
        self.y = START_Y;
        self.start = 0;
        wclear(self.win);
    }

    //H
    fn handle_movment_left(&mut self) {
        if self.x > START_X {
            self.x -= 1;
            self.idx_x -= 1;
        }
    }
    //L
    fn handle_movment_right(&mut self) {
        if self.x > self.x - 2 && self.x - START_X < self.archivo.buffer[self.idx_y].len() as i32 {
            self.x += 1;
            self.idx_x += 1;
        }
    }
    //K
    fn handle_movment_up(&mut self) {
        if self.y > START_Y {
            self.y -= 1;
            self.idx_y -= 1;
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
        } else if self.start > 0 {
            self.start -= 1;
            self.idx_y -= 1;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            wclear(self.win);
        }
    }
    //J
    fn handle_movment_down(&mut self) {
        if self.y <= self.h - 6 && self.idx_y < self.archivo.buffer.len() - 1 {
            self.y += 1;
            self.idx_y += 1;
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
        } else if self.idx_y < self.archivo.buffer.len() - 1 {
            self.start += 1;
            self.idx_y += 1;
            self.idx_x = self.archivo.buffer[self.idx_y].len();
            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
            wclear(self.win);
        }
    }
    fn handle_save(&mut self) {
        let str: String;
        let a = self.archivo.save();
        let metadata = self.archivo.file.metadata().unwrap();
        match a {
            Ok(_) => {
                str = format!(
                    "{} {}L {}B written",
                    self.archivo.path,
                    self.archivo.buffer.len(),
                    metadata.len()
                )
            }
            Err(err) => str = format!("Nothing updated due to error {}", err,),
        }

        mvwprintw(self.win, self.h - 2, 2, &str);
        wrefresh(self.win);
    }
    //B
    fn handle_start_line(&mut self) {
        self.idx_x = 0;
        self.x = START_X;
    }

    fn handle_enter(&mut self) {
        self.archivo
            .buffer
            .insert(self.idx_y + 1, Vec::<char>::new());
        self.idx_y += 1;
        self.idx_x = 0;
        self.x = START_X;

        if self.idx_y < self.archivo.buffer.len() - 1 && self.archivo.buffer.len() as i32 > self.h {
            self.start += 1;
        } else {
            self.y += 1;
        }
        wclear(self.win);
    }

    fn handle_delete(&mut self, ch: &mut i32) {
        *ch = wgetch(self.win);
        if *ch == 100 {
            if self.archivo.buffer.len() > 1 {
                if self.idx_y < 1 {
                    if !self.archivo.buffer[self.idx_y].is_empty() {
                        self.archivo.buffer[self.idx_y].clear();
                    } else {
                        self.archivo.buffer.remove(self.idx_y + 1);
                    }
                    self.x = START_X;
                    self.idx_x = 0;
                } else {
                    self.archivo.buffer.remove(self.idx_y);
                    self.idx_y -= 1;
                    self.y -= 1;
                    self.x = START_X;
                    self.idx_x = 0;
                }
            }
            wclear(self.win);
        }
        // a -> ALL
        if *ch == 97 {
            self.archivo.buffer.clear();
            self.archivo.buffer.push(Vec::<char>::new());
            self.idx_y = 0;
            self.y = START_Y;
            wclear(self.win);
        }
    }

    //I
    fn handle_insert(&mut self, ch: &mut i32) {
        self.mode = true;
        self.display_bar();
        *ch = wgetch(self.win);
        let mut ty: char;
        if *ch == 27 {
            self.mode = false;
            self.display_bar();
        } else {
            loop {
                ty = *ch as u8 as char;
                match *ch {
                    KEY_BACKSPACE => {
                        if self.x > self.archivo.buffer[self.idx_y].len() as i32 {
                            self.archivo.buffer[self.idx_y].pop();
                            self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
                            if self.idx_x > 0 {
                                self.idx_x -= 1;
                            } else {
                                if self.idx_y > 0 {
                                    self.idx_y -= 1;
                                    self.y -= 1;
                                    self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
                                    self.idx_x = self.archivo.buffer[self.idx_y].len();
                                }
                            }
                        } else {
                            self.archivo.buffer[self.idx_y].remove(self.idx_x);
                        }
                        self.display();
                    }
                    KEY_ENTER | 10 => {
                        let mut right: Vec<char> = Vec::new();
                        for i in self.idx_x..self.archivo.buffer[self.idx_y].len() {
                            right.push(self.archivo.buffer[self.idx_y][i]);
                        }
                        if let Some(buffer_y) = self.archivo.buffer.get_mut(self.idx_y) {
                            if self.idx_x < buffer_y.len() {
                                buffer_y.truncate(self.idx_x); // Elimina todos los elementos a partir del índice self.idx_x
                            }
                        }

                        self.archivo.buffer.insert(self.idx_y + 1, right);
                        self.idx_y += 1;
                        self.idx_x = 0;
                        self.x = START_X;
                        self.y += 1;
                        wclear(self.win);
                        self.display();
                    }
                    27 => {
                        self.mode = false;
                        //self.display_bar();
                        //self.display();
                    }
                    _ => {
                        if self.x < self.w - START_X {
                            if self.idx_x > self.archivo.buffer[self.idx_y].len() {
                                self.archivo.buffer[self.idx_y].push(ty);
                                // self.archivo.buffer[self.idx_y].insert(self.idx_x, ty);
                            } else {
                                self.archivo.buffer[self.idx_y].insert(self.idx_x, ty);
                            }
                            self.x += 1;
                            self.idx_x += 1;
                        } else {
                            self.idx_x = 0;
                            self.x = START_X;
                            self.idx_y += 1;
                            self.y += 1;
                        }

                        self.display();
                    }
                }
                if *ch != 27 {
                    wrefresh(self.win);
                    *ch = wgetch(self.win);
                } else {
                    self.display_bar();
                    break;
                }
            }
        }
    }
}
