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
            let (sender, receiver) = mpsc::channel::<(usize, usize)>();
            let grid_info_copy_for_display = grid_info.clone();

            let display = DisplayGrid{grid_info : grid_info_copy_for_display, receiver : Some(receiver)};
            let handle = thread::spawn(move || {
                    display.display_grid();
                });
            let wazelentin = PathDetective{grid_info, sender : Some(sender)}; 
            // IN fact the sender is only for the display, if i don't want to display anything i don't want to add a sender, change it ! -> OPTION !!!
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
