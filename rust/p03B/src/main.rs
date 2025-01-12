use std::fs;
use regex::Regex;

/*
Question 3B) Total sum of all the multiplication result.
Need to search within the puzzle input for the word "mul" with 2 numbers (1-3 digits long) separated by a comma, inside braces. eg. mul(12,345)
HOWEVER: If there is a 'don't()' instruction found, then disable the mul instruction until a do() instruction is found. Start with mul enabled.
For each "mul" found - multiply the 2 numbers and get the sum of ALL of them.
*/

// For the following entry
const TEST_ENTRIES: [&str; 1] = ["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];
// In this case, instructions mul(5,5) and mul(11,8) are disabled. So only mul(2,4) and mul(8,5) are enabled.
// gives: 2*4 + 8*5
// = 8 + 40 = 48

fn get_entries(filename: &str) -> String {
    let full_filename = format!("..\\..\\input_data\\{filename}");
    let read_text = fs::read_to_string(full_filename).expect("Failed to read input file");
    let split_text = read_text.split("\r\n");

    let mut entries_line = String::new();
    for line in split_text {
        entries_line.push_str(line);
    }
    entries_line
}

fn get_sum_for_memory_line(memory_line: &str) -> i32 {
    let mut sum = 0;
    // println!("Memory line: {:#?}", memory_line);
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mul_results = pattern.find_iter(&memory_line).map(|m| m.as_str()).collect::<Vec<&str>>();
    for mul in mul_results {
        // println!("Result = {:#?}", mul);
        let num1: i32 = mul.split(',')
                            .collect::<Vec<&str>>()[0]
                            .split('(')
                            .collect::<Vec<&str>>()[1]
                            .trim().parse().unwrap();
        let num2: i32 = mul.split(',')
                            .collect::<Vec<&str>>()[1]
                            .split(')')
                            .collect::<Vec<&str>>()[0]
                            .trim().parse().unwrap();

        // println!("{num1} * {num2}");
        sum += num1 * num2;
    }
    sum
}

fn get_sum_of_multiplication_results(memory: String) -> i32 {
    let mut sum = 0;
    let dont_str = "don't()";
    let do_str = "do()";
    
    // Start with mul enabled, hence get the block of data before the first dont() instruction
    let dont_parts = memory.split(dont_str).collect::<Vec<&str>>();
    let do_block = dont_parts[0];
    let mut dont_block = memory.split_at(do_block.len()+dont_str.len()).1;
    // calculate the mul sum for the do block
    sum += get_sum_for_memory_line(&do_block);

    loop {
        // The first part of the dont block of data we are NOT interested in.
        // The second part is the block of data we are interested in, but only up to the next dont() instruction.
        let mut parts: Vec<&str> = dont_block.split(do_str).collect();
        let next_dont_block_pos: usize;
        match parts.get(1) {
            Some(block) => {
                next_dont_block_pos = parts[0].len() + do_str.len();
                parts = block.split(dont_str).collect::<Vec<&str>>();
            },
            None => {
                break;
            }
        }
        // The first part here is the data we are interested in.
        match parts.get(0) {
            Some(block) => {
                sum += get_sum_for_memory_line(&block);
            },
            None => {
                break;
            }
        }
        // The second part here is the next dont_block
        if next_dont_block_pos >= dont_block.len() {
            break;
        }
        dont_block = dont_block.split_at(next_dont_block_pos).1;
    }
    sum
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    // In this case get entries as a single string
    let entries = get_entries("Q3input.txt");

    // Pass ownership of entries through to the "get_sum_of_multiplication_results" function
    println!("2024 - Question 3B: Total sum of all the multiplication results (taking do() and don't() into account) = {}", get_sum_of_multiplication_results(entries));
}
