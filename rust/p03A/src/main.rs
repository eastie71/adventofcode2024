use std::fs;
use regex::Regex;

/*
Question 3A) Total sum of all the multiplication result.
Need to search within the puzzle input for the word "mul" with 2 numbers (1-3 digits long) separated by a comma, inside braces. eg. mul(12,345)
For each "mul" found - multiply the 2 numbers and get the sum of ALL of them.
*/

// For the following entry
const TEST_ENTRIES: [&str; 1] = ["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];
// gives: mul(2,4) + mul(5,5) + mul(11,8) + mul(8,5)
// = 8 + 25 + 88 + 40 = 161

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

fn get_sum_of_multiplication_results(memory: Vec<String>) -> i32 {
    let mut sum = 0;
    
    let pattern = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    for memory_line in memory {
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
    }
    sum
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q3input.txt");

    // Pass ownership of entries through to the "get_sum_of_multiplication_results" function
    println!("2024 - Question 3A: Total sum of all the multiplication results = {}", get_sum_of_multiplication_results(entries));
}
