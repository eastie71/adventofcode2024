use super::xmas_letter::XmasLetter;
use crate::xmas_letter_element::XmasLetterElement;

#[derive(Debug)]
pub struct XmasWordSearch {
    grid: Vec<Vec<XmasLetterElement>>,
    rows: usize,
    cols: usize,
}

impl XmasWordSearch {
    // Create a new grid initialized with Dot elements
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::with_capacity(rows);
        for _i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for _j in 0..cols {
                row.push(XmasLetterElement::new(XmasLetter::Dot));
            }
            grid.push(row);
        }
        
        XmasWordSearch { grid, rows, cols }
    }

    // Get a reference to an element at specific position
    pub fn get(&self, row: usize, col: usize) -> Option<&XmasLetterElement> {
        self.grid.get(row)?.get(col)
    }

    // Get a mutable reference to an element at specific position
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut XmasLetterElement> {
        self.grid.get_mut(row)?.get_mut(col)
    }

    // Set a XmasLetter at specific position
    pub fn set_letter(&mut self, row: usize, col: usize, xmas_letter: XmasLetter) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Position out of bounds");
        }
        
        let element = &mut self.grid[row][col];
        element.letter = xmas_letter;
        Ok(())
    }

    // Get grid dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    // Print the grid
    pub fn display(&self) {
        for row in &self.grid {
            for element in row {
                print!("{}", element.letter.to_char());
            }
            println!();
        }
    }
}
