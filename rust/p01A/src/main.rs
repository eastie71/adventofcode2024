use std::fs;

const TEST_ENTRIES: [&str; 6] = ["3   4", "4   3", "2   5",  "1   3", "3   9", "3   3"];

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

fn get_distance_sum(values: Vec<String>) -> i32 {
    let mut sum = 0;
    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];
    
    for value in values {
        let split_values = value.split("   ").collect::<Vec<&str>>();
        left_list.push(split_values[0].trim().parse().unwrap());
        right_list.push(split_values[1].trim().parse().unwrap());
    }
    left_list.sort();
    right_list.sort();
    //println!("Left list = {:#?}", left_list);
    //println!("Right list = {:#?}", right_list);

    for i in 0..left_list.len() {
        sum += (right_list[i] - left_list[i]).abs()
    }
    sum
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q1input.txt");

    println!("2024 - Question 1A: Total distance between location IDs = {}", get_distance_sum(entries));
}
