pub mod constants;
pub mod file;
pub mod ui;
pub mod explorer;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let mut state = ui::State::new(path);
    state.display();
    state.update();
}
