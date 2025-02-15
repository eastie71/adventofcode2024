mod xmas_letter;
mod xmas_letter_element;
mod xmas_grid_search;
use std::fs;
use xmas_letter::XmasLetter;
use xmas_grid_search::XmasWordSearch;

/*
Question 4B) Calculate the total number of instances of the pattern 'X-MAS' found in the input data.

Example of X-MAS word search pattern:
M.S
.A.
M.S

where any letters that are not relevant to the word pattern 'X-MAS' are represented by a '.' character.
*/

// For the following 10 lines of text the word pattern 'X-MAS' appears 9 times
const TEST_ENTRIES: [&str; 10] =   ["MMMSXXMASM",  
                                    "MSAMXMSMSA", 
                                    "AMXSXMAAMM",  
                                    "MSAMASMSMX",  
                                    "XMASAMXAMM",  
                                    "XXAMMXXAMA",
                                    "SMSMSASXSS",
                                    "SAXAMASAAA",
                                    "MAMMMXMMMM",
                                    "MXMXAXMASX"]; 


fn get_entries(filename: &str) -> Vec<String> {
    let full_filename = format!("..\\..\\input_data\\{filename}");
    let read_text = fs::read_to_string(full_filename).expect("Failed to read input file");
    let split_text = read_text.split("\r\n");

    let mut entries = vec![];
    for line in split_text {
        entries.push(line.to_string());
    }
    entries
}

fn get_num_of_xmas_word_patterns(rows_of_text: Vec<String>) -> usize {
    let mut word_search_grid = XmasWordSearch::new(rows_of_text.len(), rows_of_text[0].len());

    for (row_idx, text_row) in rows_of_text.iter().enumerate() {
        for (col_idx, letter) in text_row.chars().enumerate() {
            let valid_letter = XmasLetter::from_char(letter).ok_or("Invalid xmas letter found");
            word_search_grid.set_letter(row_idx, col_idx, valid_letter.unwrap()).unwrap_or_else(|_| panic!("Failed to set xmas letter"));
        }
    }

    word_search_grid.display();

    // Calculate the total number of instances of the pattern 'X-MAS'
    word_search_grid.set_word_pattern_count();
    word_search_grid.get_total_word_pattern_count(false)
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q4input.txt");

    // Pass ownership of entries through to the "get_num_of_xmas_word_patterns" function
    println!("2024 - Question 4B: Total number of instances of the word pattern X-MAS found = {}", get_num_of_xmas_word_patterns(entries));
}
