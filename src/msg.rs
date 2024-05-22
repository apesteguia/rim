use ncurses::*;

#[derive(Debug)]
pub enum MsgType {
    Error,
    Info,
}

#[derive(Debug)]
pub struct Msg {
    pub txt: String,
    pub typ: MsgType,
    pub win: WINDOW,
}

impl Msg {
    pub fn new(win: WINDOW, txt: impl Into<String>, typ: MsgType) -> Self {
        Self {
            win,
            txt: txt.into(),
            typ,
        }
    }

    pub fn update(&mut self, txt: impl Into<String>, typ: MsgType) {
        self.txt = txt.into();
        self.typ = typ;
    }

    pub fn display(&self, x: i32, y: i32) {
        match self.typ {
            MsgType::Info => {
                wattron(self.win, COLOR_PAIR(1));
                mvwprintw(self.win, y, x, &self.txt);
                wattroff(self.win, COLOR_PAIR(1));
            }
            MsgType::Error => {
                wattron(self.win, COLOR_PAIR(5));
                mvwprintw(self.win, y, x, &self.txt);
                wattroff(self.win, COLOR_PAIR(5));
            }
        };
    }
}
