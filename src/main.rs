pub mod constants;
pub mod file;
pub mod ui;

use std::env;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let path = &args[1];
    let mut state = ui::State::new(path);
    state.display();
    state.update();
}
