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

    // Set a XmasLetter at specific position
    pub fn set_letter(&mut self, row: usize, col: usize, xmas_letter: XmasLetter) -> Result<(), &'static str> {
        if row >= self.rows || col >= self.cols {
            return Err("Position out of bounds");
        }
        
        let element = &mut self.grid[row][col];
        element.letter = xmas_letter;
        Ok(())
    }

    pub fn increment_word_count(&mut self, row: usize, col: usize) {
        self.grid[row][col].word_count += 1;
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

    /*
     * Looking for a X-MAS pattern in the grid, that looks like this example:
     * M.S
     * .A.
     * M.S
     */
    fn check_xmas_pattern(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx < 1 || cidx < 1 || cidx > grid[0].len() - 2  || ridx > grid.len() - 2 {
            return false;
        }
        ((grid[ridx - 1][cidx-1].letter == XmasLetter::M && grid[ridx + 1][cidx+1].letter == XmasLetter::S) ||
         (grid[ridx - 1][cidx-1].letter == XmasLetter::S && grid[ridx + 1][cidx+1].letter == XmasLetter::M))
         &&
        ((grid[ridx - 1][cidx+1].letter == XmasLetter::M && grid[ridx + 1][cidx-1].letter == XmasLetter::S) ||
         (grid[ridx - 1][cidx+1].letter == XmasLetter::S && grid[ridx + 1][cidx-1].letter == XmasLetter::M))
    }

    // Use XmasLetter::A as the base letter to search for the word pattern 'X-MAS'
    pub fn set_word_pattern_count(&mut self) {
        for ridx in 0..self.rows {
            for cidx in 0..self.cols {
                if self.grid[ridx][cidx].letter == XmasLetter::A {
                    if Self::check_xmas_pattern(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                }
            }
        }
    }

    pub fn get_total_word_pattern_count(&self, print: bool) -> usize {
        let mut total = 0;
        for (ridx, row) in self.grid.iter().enumerate() {
            for (cidx, element) in row.iter().enumerate() {
                total += element.word_count;
                if print && element.word_count > 0 {
                    println!("({},{}): {}", ridx, cidx, element.word_count);
                }
            }
        }
        total
    }
}
