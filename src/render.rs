use std::io::Stdout;
use std::io::Write;
use crossterm::cursor::MoveTo;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, current_frame: &Frame, force: bool)
{
    if force
    {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap(); // Unwrap returns the value when called on Result if there is one and panics when there isn't. With Options, it will return Some value, or if it's none, panic.
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    for (x, column) in current_frame.iter().enumerate()
    {
        for (y, s) in column.iter().enumerate()
        {
            if *s != last_frame[x][y] || force
            {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }

    stdout.flush().unwrap();
}