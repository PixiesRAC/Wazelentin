use crate::GridInfo;
use std::io::Write;
use std::sync::mpsc;
use std::time::Duration;
use termion::{clear, color, cursor};

pub struct DisplayGrid {
    pub grid_info: GridInfo,
    pub receiver: Option<mpsc::Receiver<(usize, usize)>>,
}
impl DisplayGrid {
    pub fn display_grid(&self) {
        if self.receiver.is_none() {
            return;
        }
        print!("{}", clear::All);
        let mut x = 1;
        let mut y = 1;
        for lines in self.grid_info.grid.iter().rev() {
            for case in lines {
                if case == &GridInfo::CASE_WIN {
                    print!("{}{}A", cursor::Goto(x, y), color::Fg(color::Green));
                } else if case == &GridInfo::CASE_CLOSE {
                    print!("{}{}{}", cursor::Goto(x, y), color::Fg(color::Red), case);
                } else {
                    print!(
                        "{}{}{}",
                        cursor::Goto(x, y),
                        color::Fg(color::LightBlue),
                        case
                    );
                }
                std::io::stdout().flush().unwrap();
                x += 1;
            }
            x = 1;
            y += 1;
        }
        let start_x = (self.grid_info.start_pos.1 as u16) + 1;
        let start_y = (self.grid_info.row_max as u16) - (self.grid_info.start_pos.0 as u16) + 1;
        print!(
            "{}{}B",
            cursor::Goto(start_x, start_y),
            color::Fg(color::Green)
        );
        print!("\n\n\n{}", cursor::Save);

        loop {
            if let Some(receiver) = &self.receiver {
                if let Ok(case) = receiver.recv() {
                    let new_x = case.1 as u16 + 1;
                    let mut new_y = case.0 as u16;

                    if case == self.grid_info.start_pos {
                        print!("{}", cursor::Restore);
                        print!("{}", color::Fg(color::Reset));
                        print!("{}", cursor::Show);
                        std::io::stdout().flush().unwrap();
                        break;
                    }
                    new_y = (self.grid_info.row_max as u16) - new_y + 1;
                    print!(
                        "{}{}{}@",
                        cursor::Goto(new_x, new_y),
                        color::Fg(color::Yellow),
                        cursor::Hide
                    );
                    std::thread::sleep(Duration::from_millis(100));
                }
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
