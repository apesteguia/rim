pub mod constants;
pub mod explorer;
pub mod file;
pub mod ui;

use std::env;

use file::is_file;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let mut explorer = explorer::Explorer::raw("");
        explorer.get_files()?;
        explorer.display();
        let a = explorer.update_raw();
        match a {
            Some(p) => {
                let mut state = ui::State::new(&p);
                state.display();
                state.update();
            }
            None => (),
        }
    } else {
        if !is_file(&args[1]) {
            let mut explorer = explorer::Explorer::raw(&args[1]);
            explorer.get_files()?;
            explorer.display();
            let a = explorer.update_raw();
            match a {
                Some(p) => {
                    let mut state = ui::State::new(&p);
                    state.display();
                    state.update();
                }
                None => (),
            }
        } else {
            let mut state = ui::State::new(&args[1]);
            state.display();
            state.update();
        }
    }

    Ok(())
}
