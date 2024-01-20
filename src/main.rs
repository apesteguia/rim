pub mod file;
pub mod ui;
fn main() {
    let path = "/home/mikel/main.c";
    let mut state = ui::State::new(path, 10000);
    state.display();
    state.update();
}
