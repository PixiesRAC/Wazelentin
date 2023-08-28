use crate::GridInfo;
use std::time::Duration;
use std::sync::mpsc;
use termion::{color, cursor, style, clear};
use std::io::{self, Write};


pub struct DisplayGrid {
    pub grid_info: GridInfo,
    pub receiver : mpsc::Receiver<(usize, usize)>
}
impl DisplayGrid {
    pub fn display_grid(&self) {

            print!("{}", clear::All);
            let mut  x = 1;
            let mut  y = 1;
            for lines in self.grid_info.grid.iter().rev() {
            for case in lines {
                if case == &GridInfo::CASE_WIN
                {
                    print!("{}{}{}", cursor::Goto(x, y), color::Fg(color::Green), "A");
                }
                else if case == &GridInfo::CASE_CLOSE
                {
                    print!("{}{}{}", cursor::Goto(x, y), color::Fg(color::Red), case);
                }
                else {
                    print!("{}{}{}", cursor::Goto(x, y), color::Fg(color::LightBlue), case);
                }
                std::io::stdout().flush().unwrap();
                x += 1;
            }
            x = 1;
            y += 1;
        }
        let mut start_x = (self.grid_info.start_pos.1 as u16) + 1;
        let mut start_y = (self.grid_info.row_max as u16) - (self.grid_info.start_pos.0 as u16) + 1;
        print!("{}{}{}", cursor::Goto(start_x, start_y), color::Fg(color::Green), "B");
        print!("\n\n\n{}", cursor::Save);

        loop {
            
            if let Ok(case) = self.receiver.recv()
            {
                let mut new_x = case.1 as u16 + 1;
                let mut new_y = case.0 as u16;

                if case == self.grid_info.start_pos
                {
                    print!("{}", cursor::Restore);
                    print!("{}", color::Fg(color::Reset));
                    print!("{}", cursor::Show);
                    std::io::stdout().flush().unwrap();
                    break;
                }
                new_y = (self.grid_info.row_max as u16) - new_y + 1;
                print!("{}{}{}{}", cursor::Goto(new_x, new_y), color::Fg(color::Yellow), cursor::Hide, "@");
                std::thread::sleep(Duration::from_millis(500));
            }
            std::io::stdout().flush().unwrap();
        }
    }
}
