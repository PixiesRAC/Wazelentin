use clap::Parser;
mod grid_info;

use grid_info::GridInfo;

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
    if let Ok(_) = GridInfo::new(&grid_row_file)
    {
        
    }

    Ok(())
}
