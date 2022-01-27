use crate::{NUM_COLS, NUM_ROWS};

pub type Frame = Vec<Vec<&'static str>>; // 'static means that the data pointed lives for the entire liftime of the running program.

pub fn new_frame() -> Frame
{
    let mut columns = Vec::with_capacity(NUM_COLS);
    for _ in 0..NUM_COLS
    {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS
        {
            col.push(" "); // Create a new frame with our logic every time around the game loop.
        }

        columns.push(col);
    }

    columns
}

pub trait Drawable
{
    fn draw(&self, frame: &mut Frame);
} 
