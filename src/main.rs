mod grid_info;
mod path_detective;
mod grid_display;

use std::thread;
use clap::Parser;
use std::sync::mpsc;

use grid_info::GridInfo;
use path_detective::PathDetective;
use grid_display::DisplayGrid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to "map file"
    #[arg(short, long)]
    grid_file: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid_file: String = Args::parse().grid_file;
    println!("Lets run Wazelentin on [{}]", grid_file);
    let grid_row_file: String = std::fs::read_to_string(grid_file)?;
    match GridInfo::new(&grid_row_file)
    {
        Ok(grid_info) =>
        {
            let (sender, receiver) : (mpsc::Sender<(usize, usize)>, mpsc::Receiver<(usize, usize)>)= mpsc::channel();

            let grid_info_copy_for_display = grid_info.clone();

            let display = DisplayGrid{grid_info : grid_info_copy_for_display, receiver : receiver};
            let handle = thread::spawn(move || {
                let (sender, receiver) : (mpsc::Sender<(usize, usize)>, mpsc::Receiver<(usize, usize)>)= mpsc::channel();
                    display.display_grid();
                });
            let wazelentin = PathDetective{grid_info : grid_info, sender : sender};
            wazelentin.find_and_transmit_shortest_path();
            handle.join().unwrap();

        }
        Err(err) =>
        {
            eprintln!("An error has occured [{:?}]", err);
        }
    }
    Ok(())
}
