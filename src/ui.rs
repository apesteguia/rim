use std::{fs, os::unix::fs::PermissionsExt};

use crate::file::Archivo;

use ncurses::*;

#[derive(Debug)]
enum Lenguaje {
    Rust,
    Elixir,
    C,
    Cpp,
    JavaScript,
    TypeScript,
    Java,
    Lua,
    Python,
    Txt,
    Markdown,
    Jsx,
    Assembly,
    Haskell,
    OCaml,
    Clojure,
    Go,
    Css,
    Html,
    Bash,
    Php,
    Ruby,
    Undefined,
}

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
        init_pair(2, COLOR_BLACK, COLOR_BLUE);
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
            x: 5,
            y: 1,
            mode: false,
            idx_x: 0,
            idx_y: 0,
            start: 0,
            end,
        }
    }

    pub fn display(&self) {
        wclear(self.win);
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
                    "{}  ",
                    self.archivo.buffer[i as usize]
                        .iter()
                        .cloned()
                        .collect::<String>()
                )
            } else if i < 100 {
                format!(
                    "{} ",
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
            mvwprintw(self.win, (_idx + 1) as i32, 5, &format);
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

    pub fn update(&mut self) {
        keypad(self.win, true);

        let mut ch = wgetch(self.win);
        while ch != 113 {
            self.mode = false;
            match ch {
                //J
                106 => {
                    if self.y <= self.h - 7 && self.y < self.archivo.buffer.len() as i32 {
                        self.y += 1;
                        self.idx_y += 1;
                    } else if self.start <= self.h + 9 {
                        self.start += 1;
                    }
                }
                //K
                107 => {
                    if self.y > 1 {
                        self.y -= 1;
                        self.idx_y -= 1;
                    } else if self.start > 0 {
                        self.start -= 1;
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
                    self.mode = !self.mode;
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
                            KEY_ENTER => {
                                self.y -= 1;
                                self.idx_y -= 1;
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

fn obtener_nombre_lenguaje(codigo: &str) -> Option<Lenguaje> {
    match codigo.to_lowercase().as_str() {
        "rs" => Some(Lenguaje::Rust),
        "ex" | "exs" => Some(Lenguaje::Elixir),
        "c" | "h" => Some(Lenguaje::C),
        "cpp" | "c++" | "hpp" => Some(Lenguaje::Cpp),
        "js" => Some(Lenguaje::JavaScript),
        "ts" => Some(Lenguaje::TypeScript),
        "java" => Some(Lenguaje::Java),
        "lua" => Some(Lenguaje::Lua),
        "py" => Some(Lenguaje::Python),
        "txt" => Some(Lenguaje::Txt),
        "md" | "mdx" => Some(Lenguaje::Markdown),
        "jsx" | "tsx" => Some(Lenguaje::Jsx),
        "s" | "asm" | "nasm" => Some(Lenguaje::Assembly),
        "hs" => Some(Lenguaje::Haskell),
        "ml" => Some(Lenguaje::OCaml),
        "cjl" => Some(Lenguaje::Clojure),
        "go" => Some(Lenguaje::Go),
        "css" => Some(Lenguaje::Css),
        "html" => Some(Lenguaje::Html),
        "sh" => Some(Lenguaje::Bash),
        "php" => Some(Lenguaje::Php),
        "rb" => Some(Lenguaje::Ruby),
        _ => Some(Lenguaje::Undefined),
    }
}
