pub mod file;
pub mod ui;
fn main() {
    let path = "/home/mikel/main.rs";
    let mut state = ui::State::new(path);
    state.display();
    state.update();
}
