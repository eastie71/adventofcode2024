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

    // // Get a reference to an element at specific position
    // pub fn get(&self, row: usize, col: usize) -> Option<&XmasLetterElement> {
    //     self.grid.get(row)?.get(col)
    // }

    // // Get a mutable reference to an element at specific position
    // pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut XmasLetterElement> {
    //     self.grid.get_mut(row)?.get_mut(col)
    // }

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

    // // Get grid dimensions
    // pub fn dimensions(&self) -> (usize, usize) {
    //     (self.rows, self.cols)
    // }

    // Print the grid
    pub fn display(&self) {
        for row in &self.grid {
            for element in row {
                print!("{}", element.letter.to_char());
            }
            println!();
        }
    }

    // Check if the word 'XMAS' is found in the North direction
    fn check_n(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx < 3 {
            return false;
        }
        grid[ridx - 1][cidx].letter == XmasLetter::M
            && grid[ridx - 2][cidx].letter == XmasLetter::A
            && grid[ridx - 3][cidx].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the North-East direction
    fn check_ne(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx < 3 || cidx > grid[0].len() - 4 {
            return false;
        }
        grid[ridx - 1][cidx + 1].letter == XmasLetter::M
            && grid[ridx - 2][cidx + 2].letter == XmasLetter::A
            && grid[ridx - 3][cidx + 3].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the East direction
    fn check_e(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if cidx > grid[0].len() - 4 {
            return false;
        }
        grid[ridx][cidx + 1].letter == XmasLetter::M
            && grid[ridx][cidx + 2].letter == XmasLetter::A
            && grid[ridx][cidx + 3].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the South-East direction
    fn check_se(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx > grid.len() - 4 || cidx > grid[0].len() - 4 {
            return false;
        }
        grid[ridx + 1][cidx + 1].letter == XmasLetter::M
            && grid[ridx + 2][cidx + 2].letter == XmasLetter::A
            && grid[ridx + 3][cidx + 3].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the South direction
    fn check_s(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx > grid.len() - 4 {
            return false;
        }
        grid[ridx + 1][cidx].letter == XmasLetter::M
            && grid[ridx + 2][cidx].letter == XmasLetter::A
            && grid[ridx + 3][cidx].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the South-West direction
    fn check_sw(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx > grid.len() - 4 || cidx < 3 {
            return false;
        }
        grid[ridx + 1][cidx - 1].letter == XmasLetter::M
            && grid[ridx + 2][cidx - 2].letter == XmasLetter::A
            && grid[ridx + 3][cidx - 3].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the West direction
    fn check_w(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if cidx < 3 {
            return false;
        }
        grid[ridx][cidx - 1].letter == XmasLetter::M
            && grid[ridx][cidx - 2].letter == XmasLetter::A
            && grid[ridx][cidx - 3].letter == XmasLetter::S
    }

    // Check if the word 'XMAS' is found in the North-West direction
    fn check_nw(grid: &Vec<Vec<XmasLetterElement>>, ridx: usize, cidx: usize) -> bool {
        if ridx < 3 || cidx < 3 {
            return false;
        }
        grid[ridx - 1][cidx - 1].letter == XmasLetter::M
            && grid[ridx - 2][cidx - 2].letter == XmasLetter::A
            && grid[ridx - 3][cidx - 3].letter == XmasLetter::S
    }

    pub fn set_word_count(&mut self) {
        for ridx in 0..self.rows {
            for cidx in 0..self.cols {
                if self.grid[ridx][cidx].letter == XmasLetter::X {
                    if Self::check_n(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_ne(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_e(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_se(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_s(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_sw(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_w(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                    if Self::check_nw(&self.grid, ridx, cidx) {
                        self.increment_word_count(ridx, cidx);
                    }
                }
            }
        }
    }

    pub fn get_total_word_count(&self, print: bool) -> usize {
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
