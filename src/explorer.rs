use crate::{
    file::is_file,
    msg::{Msg, MsgType},
};
use ncurses::*;
use std::fs;

const START_X: i32 = 1;
const START_Y: i32 = 1;

#[derive(Debug, Clone, PartialEq)]
pub struct Dir {
    pub path: String,
    pub isfile: bool,
}

impl Dir {
    pub fn new(path: impl Into<String>, isfile: bool) -> Self {
        Self {
            path: path.into(),
            isfile,
        }
    }
}

#[derive(Debug)]
pub struct Explorer {
    pub path: String,
    pub dirs: Vec<Dir>,
    pub w: i32,
    pub h: i32,
    pub win: WINDOW,
    pub x: i32,
    pub y: i32,
    pub selected: usize,
    pub idx_x: i32,
    pub idx_y: i32,
    pub start: i32,
    pub end: i32,
    pub msg: Msg,
}

impl Explorer {
    pub fn new(path: impl Into<String> + Copy) -> Self {
        init_color(COLOR_BLACK as i16, 90, 90, 90);
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        init_pair(2, COLOR_WHITE, COLOR_BLUE);
        init_pair(3, COLOR_BLUE, COLOR_BLACK);
        init_pair(4, COLOR_RED, COLOR_BLACK);

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
        if p.is_empty() {
            p = std::env::current_dir()
                .expect("FAILED CURRENT DIR")
                .to_str()
                .expect("FAILED CONVERT TO STRING")
                .to_string();
        } else if !is_file(&p) {
        } else {
            if let Ok(current_dir) = std::env::current_dir() {
                let file_path = current_dir.join(p);
                let buf = std::path::Path::new(&file_path);
                if let Some(parent_dir) = buf.parent() {
                    p = parent_dir.display().to_string();
                } else {
                    p = "NO DATA".to_string();
                }
            }
        }

        Self {
            path: p,
            dirs: Vec::new(),
            w: width_with_margin as i32,
            h: height_with_margin as i32,
            win,
            selected: 0,
            x: START_X,
            y: START_Y,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end: max_height - 5,
            msg: Msg::new(win, "", MsgType::Info),
        }
    }

    pub fn raw(path: impl Into<String> + Copy) -> Self {
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
        init_pair(4, COLOR_RED, COLOR_BLACK);
        init_pair(5, COLOR_BLACK, COLOR_RED);

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
        if p.is_empty() {
            p = std::env::current_dir()
                .expect("FAILED CURRENT DIR")
                .to_str()
                .expect("FAILED CONVERT TO STRING")
                .to_string();
        } else {
            if let Ok(current_dir) = std::env::current_dir() {
                let file_path = current_dir.join(p);
                let buf = std::path::Path::new(&file_path);
                if let Some(parent_dir) = buf.parent() {
                    p = parent_dir.display().to_string();
                } else {
                    p = "NO DATA".to_string();
                }
            }
        }

        Self {
            path: p,
            dirs: Vec::new(),
            w: width_with_margin as i32,
            h: height_with_margin as i32,
            win,
            selected: 0,
            x: START_X,
            y: START_Y,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end: max_height - 5,
            msg: Msg::new(win, "", MsgType::Info),
        }
    }

    pub fn display(&self) {
        wclear(self.win);
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        box_(self.win, 0, 0);
        wattron(self.win, COLOR_PAIR(2) | A_BOLD());
        mvwprintw(self.win, 0, 1, &self.path);
        wattroff(self.win, COLOR_PAIR(2) | A_BOLD());

        let mut counter = 0;
        for i in self.selected..self.dirs.len() {
            if self.selected == i {
                wattron(self.win, COLOR_PAIR(1) | A_BOLD());
                mvwprintw(self.win, counter + self.y, self.x, &self.dirs[i].path);
                wattroff(self.win, COLOR_PAIR(1) | A_BOLD());
            } else {
                mvwprintw(self.win, counter + self.y, self.x, &self.dirs[i].path);
            }
            counter += 1;
        }

        self.msg.display(1, self.h - 2);
        wrefresh(self.win);
    }

    pub fn update(&mut self) -> Option<String> {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        loop {
            match ch {
                113 => break,
                106 => {
                    if self.selected < self.dirs.len() - 1 {
                        self.selected += 1;
                    }
                }
                108 => {
                    self.selected = 0;
                    if !self.dirs[self.selected].isfile {
                        let mut p = std::path::PathBuf::from(&self.path);
                        p.push(&self.dirs[self.selected].path);
                        self.path = p.to_str().unwrap().to_string();
                        self.dirs.clear();
                        self.get_files().unwrap();
                    }
                }
                104 => {
                    self.selected = 0;
                    let p = std::path::PathBuf::from(&self.path);
                    let parent = p.parent();
                    match parent {
                        Some(parent) => {
                            self.path = parent.to_str().unwrap().to_string();
                            self.dirs.clear();
                            self.get_files().unwrap();
                        }
                        None => {
                            self.msg.update("Can't acces dir", MsgType::Error);
                        }
                    }
                }

                107 => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KEY_ENTER | 10 | 111 => {
                    if self.dirs[self.selected].isfile {
                        return Some(self.dirs[self.selected].path.clone());
                    }
                }
                _ => {
                    self.x += 0;
                }
            }
            self.display();
            ch = wgetch(self.win);
        }

        None
    }

    pub fn update_raw(&mut self) -> Option<String> {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        loop {
            match ch {
                113 => break,
                106 => {
                    if self.selected < self.dirs.len() - 1 {
                        self.selected += 1;
                    }
                }
                108 => {
                    if !self.dirs[self.selected].isfile {
                        let mut p = std::path::PathBuf::from(&self.path);
                        p.push(&self.dirs[self.selected].path);
                        self.path = p.to_str().unwrap().to_string();
                        self.dirs.clear();
                        self.get_files().unwrap();
                    }
                    self.selected = 0;
                }
                104 => {
                    self.selected = 0;
                    let p = std::path::PathBuf::from(&self.path);
                    let parent = p.parent();
                    match parent {
                        Some(parent) => {
                            self.path = parent.to_str().unwrap().to_string();
                            self.dirs.clear();
                            self.get_files().unwrap();
                        }
                        None => self.msg.update("Can't acces dir", MsgType::Error),
                    }
                }
                107 => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    }
                }
                KEY_ENTER | 10 | 111 => {
                    if self.dirs[self.selected].isfile {
                        return Some(self.dirs[self.selected].path.clone());
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
        None
    }

    pub fn get_files(&mut self) -> Result<(), std::io::Error> {
        let paths = fs::read_dir(&self.path);
        match paths {
            Ok(paths) => {
                for entry in paths {
                    let entry = entry;
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                self.dirs.push(Dir::new(path.display().to_string(), false));
                            } else {
                                self.dirs.push(Dir::new(path.display().to_string(), true));
                            }
                        }
                        Err(err) => return Err(err),
                    }
                }
            }
            Err(err) => self.msg.update(err.to_string(), MsgType::Error),
        }
        Ok(())
    }

    // pub fn get_dirs(&mut self) -> Result<(), std::io::Error> {
    //     let paths = fs::read_dir(&self.path)?;
    //     for i in paths {
    //         self.dirs.push(i?.path().display().to_string());
    //     }
    //     Ok(())
    // }
}
