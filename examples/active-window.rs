use active_win_pos_rs::get_active_window;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    match get_active_window() {
        Ok(active_window) => {
            println!("active window: {:#?}", active_window);
        }
        Err(()) => {
            println!("error occurred while getting the active window");
        }
    }
    println!("time elapsed: {:?}", start.elapsed());
}
