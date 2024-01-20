use std::{fs, os::unix::fs::PermissionsExt};

use crate::file::Archivo;

use ncurses::*;

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
}

impl State {
    pub fn new(path: &str) -> State {
        initscr();
        noecho();
        keypad(stdscr(), true);
        raw();
        start_color();
        cbreak();
        init_pair(1, COLOR_BLUE, COLOR_WHITE);

        let w = getmaxx(stdscr());
        let h = getmaxy(stdscr());

        let win = newwin(h, w, 0, 0);

        State {
            archivo: Archivo::new(path),
            w,
            h,
            win,
            x: 6,
            y: 1,
            mode: false,
            idx_x: 0,
            idx_y: 0,
        }
    }

    pub fn display(&self) {
        wclear(self.win);
        box_(self.win, 0, 0);

        mvwprintw(self.win, 0, 1, &self.archivo.path);

        for (i, f) in self.archivo.buffer.iter().enumerate() {
            if i as i32 >= self.h - 5 {
                break;
            }

            let format = if i < 10 {
                format!(" {} | {}", i, f.iter().cloned().collect::<String>())
            } else {
                format!("{} | {}", i, f.iter().cloned().collect::<String>())
            };

            mvwprintw(self.win, (i + 1) as i32, 1, &format);
        }

        let metadata = self.archivo.file.metadata().unwrap();
        let per = format_permissions(metadata.permissions(), false);

        let format = format!(
            "Size: {} KB | {} x:{} y:{} realx:{} realy:{} char:{}",
            metadata.len(),
            per,
            self.x,
            self.y,
            self.idx_x,
            self.idx_y,
            self.archivo.buffer[9].len()
        );
        mvwprintw(self.win, self.h - 3, 1, &format);

        mvwhline(self.win, self.h - 5, 1, 95, self.w - 2);
        wmove(self.win, self.y, self.x);

        wrefresh(self.win);
    }

    pub fn update(&mut self) {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        while ch != 113 {
            match ch {
                //J
                106 => {
                    if self.y <= self.h - 7 {
                        self.y += 1;
                        self.idx_y += 1;
                    }
                }
                //K
                107 => {
                    if self.y > 1 {
                        self.y -= 1;
                        self.idx_y -= 1;
                    }
                }
                //H
                104 => {
                    if self.x > 6 {
                        self.x -= 1;
                        self.idx_x -= 1;
                    }
                }
                // L
                108 => {
                    if self.x > self.x - 2 {
                        self.x += 1;
                        self.idx_x += 1;
                    }
                }
                //g
                103 => {
                    self.archivo.save();
                }
                //insert
                105 => {
                    ch = wgetch(self.win);
                    let mut ty: char;
                    while ch != 97 {
                        ty = ch as u8 as char;
                        match ch {
                            KEY_BACKSPACE => {
                                if self.x > self.archivo.buffer[self.idx_y].len() as i32 {
                                    self.archivo.buffer[self.idx_y].pop();
                                    wmove(
                                        self.win,
                                        self.y,
                                        self.archivo.buffer[self.idx_y].len() as i32,
                                    );
                                } else {
                                    self.archivo.buffer[self.idx_y].remove(self.idx_x);
                                    //wmove(self.win, self.y, self.x);
                                }
                                //self.x -= 1;
                            }
                            _ => {
                                if self.idx_x > self.archivo.buffer[self.idx_y].len() {
                                    self.archivo.buffer[self.idx_y].push(ty);
                                } else {
                                    self.archivo.buffer[self.idx_y].insert(self.idx_x, ty);
                                }
                                self.x += 1;
                                self.idx_x += 1;
                            }
                        }
                        self.display();
                        ch = wgetch(self.win);
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

pub fn format_permissions(permissions: fs::Permissions, is_directory: bool) -> String {
    let mode = permissions.mode();

    let file_type_char = if is_directory { 'd' } else { '-' };

    let owner_read = if mode & 0o400 != 0 { 'r' } else { '-' };
    let owner_write = if mode & 0o200 != 0 { 'w' } else { '-' };
    let owner_execute = if mode & 0o100 != 0 { 'x' } else { '-' };

    let group_read = if mode & 0o040 != 0 { 'r' } else { '-' };
    let group_write = if mode & 0o020 != 0 { 'w' } else { '-' };
    let group_execute = if mode & 0o010 != 0 { 'x' } else { '-' };

    let other_read = if mode & 0o004 != 0 { 'r' } else { '-' };
    let other_write = if mode & 0o002 != 0 { 'w' } else { '-' };
    let other_execute = if mode & 0o001 != 0 { 'x' } else { '-' };

    format!(
        "{}{}{}{}{}{}{}{}{}{}",
        file_type_char,
        owner_read,
        owner_write,
        owner_execute,
        group_read,
        group_write,
        group_execute,
        other_read,
        other_write,
        other_execute
    )
}
