use crate::constants::obtener_nombre_lenguaje;
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
}

impl State {
    pub fn new(path: &str) -> State {
        initscr();
        noecho();
        keypad(stdscr(), true);
        raw();
        start_color();
        cbreak();
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);

        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());
        let end = h - 5;

        let win = newwin(h, w, 0, 0);

        State {
            archivo: Archivo::new(path),
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
        }
    }

    pub fn display(&self) {
        //wclear(self.win);
        //box_(self.win, 0, 0);

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

        /*
        for (i, f) in self.archivo.buffer.iter().enumerate() {
            if i as i32 >= self.h - 5 {
                break;
            }

            if i as i32 <= self.start {
                let format = if i < 10 {
                    format!(" {} | {}", i, f.iter().cloned().collect::<String>())
                } else {
                    format!("{} | {}", i, f.iter().cloned().collect::<String>())
                };

                mvwprintw(self.win, (i + 1) as i32, 1, &format);
            }
        }
        */

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
            self.y,
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
            self.y,
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
                //d
                100 => {
                    ch = wgetch(self.win);
                    if ch == 100 {
                        if self.archivo.buffer.len() > 1 {
                            if self.idx_y < 1 {
                                self.archivo.buffer.remove(self.idx_y + 1);
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
                    if ch == 97 {
                        self.archivo.buffer.clear();
                        self.archivo.buffer.push(Vec::<char>::new());
                        self.idx_y = 0;
                        self.y = START_Y;
                        wclear(self.win);
                    }
                }
                //J
                106 => {
                    if self.y <= self.h - 6 && self.idx_y < self.archivo.buffer.len() - 1 {
                        self.y += 1;
                        self.idx_y += 1;
                        self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
                        self.idx_x = self.archivo.buffer[self.idx_y].len();
                    } else if self.idx_y < self.archivo.buffer.len() - 1 {
                        self.start += 1;
                        self.idx_y += 1;
                        self.idx_x = self.archivo.buffer[self.idx_y].len();
                        wclear(self.win);
                    }
                }
                //K
                107 => {
                    if self.y > START_Y {
                        self.y -= 1;
                        self.idx_y -= 1;
                        self.x = self.archivo.buffer[self.idx_y].len() as i32 + START_X;
                        self.idx_x = self.archivo.buffer[self.idx_y].len();
                    } else if self.start > 0 {
                        self.start -= 1;
                        self.idx_y -= 1;
                        self.idx_x = self.archivo.buffer[self.idx_y].len();
                        wclear(self.win);
                    }
                }
                //H
                104 => {
                    if self.x > START_X {
                        self.x -= 1;
                        self.idx_x -= 1;
                    }
                }
                // L
                108 => {
                    if self.x > self.x - 2
                        && self.x - START_X < self.archivo.buffer[self.idx_y].len() as i32
                    {
                        self.x += 1;
                        self.idx_x += 1;
                    }
                }
                98 => {
                    self.idx_x = 0;
                    self.x = START_X;
                }
                KEY_ENTER | 10 | 111 => {
                    self.archivo
                        .buffer
                        .insert(self.idx_y + 1, Vec::<char>::new());
                    self.idx_y += 1;
                    self.idx_x = 0;
                    self.x = START_X;

                    if self.idx_y < self.archivo.buffer.len() - 1
                        && self.archivo.buffer.len() as i32 > self.h
                    {
                        self.start += 1;
                    } else {
                        self.y += 1;
                    }
                    wclear(self.win);
                }
                //g
                103 => {
                    self.archivo.save();
                }
                //insert i
                105 => {
                    self.mode = true;
                    self.display_bar();
                    ch = wgetch(self.win);
                    let mut ty: char;
                    if ch == 27 {
                        self.mode = false;
                        self.display_bar();
                    } else {
                        loop {
                            ty = ch as u8 as char;
                            match ch {
                                KEY_BACKSPACE => {
                                    if self.x > self.archivo.buffer[self.idx_y].len() as i32 {
                                        self.archivo.buffer[self.idx_y].pop();
                                        self.x =
                                            self.archivo.buffer[self.idx_y].len() as i32 + START_X;
                                        if self.idx_x > 0 {
                                            self.idx_x -= 1;
                                        }
                                    } else {
                                        self.archivo.buffer[self.idx_y].remove(self.idx_x);
                                    }
                                    self.display();
                                }
                                KEY_ENTER | 10 => {
                                    self.archivo
                                        .buffer
                                        .insert(self.idx_y + 1, Vec::<char>::new());
                                    self.idx_y += 1;
                                    self.idx_x = 0;
                                    self.x = START_X;
                                    self.y += 1;
                                    wclear(self.win);
                                    self.display();
                                }
                                27 => {
                                    self.mode = false;
                                    self.display_bar();
                                }
                                _ => {
                                    if self.idx_x > self.archivo.buffer[self.idx_y].len() {
                                        self.archivo.buffer[self.idx_y].push(ty);
                                        // self.archivo.buffer[self.idx_y].insert(self.idx_x, ty);
                                    } else {
                                        self.archivo.buffer[self.idx_y].insert(self.idx_x, ty);
                                    }
                                    self.x += 1;
                                    self.idx_x += 1;
                                    self.display();
                                }
                            }
                            if ch != 27 {
                                wrefresh(self.win);
                                ch = wgetch(self.win);
                            } else {
                                self.display_bar();
                                break;
                            }
                        }
                    }
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
}
