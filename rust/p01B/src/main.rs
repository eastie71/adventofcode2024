use std::fs;

/*
Question 1B) Calculate the total similarity score
To calculate the total similarity score you multiply each number in the LEFT list by the number of times that number appears in the RIGHT list, and sum all together.
*/
// For the following 6 lines, values are: (3 * 3), (4 * 1), (2 * 0), (1 * 0), (3 * 3), (3 * 3). ie. 9 + 4 + 0 + 0 + 9 + 9 = 31 
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

fn get_similarity_score(values: Vec<String>) -> i32 {
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

    let mut last_id = -1;
    let mut last_score = 0;
    for i in 0..left_list.len() {
        if last_id != left_list[i] {
            let mut count = 0;
            for right_num in &right_list {
                // count number of times left list number appears in the right list
                if left_list[i] == *right_num {
                    count += 1;
                }
            }
            last_score = left_list[i] * count;
            last_id = left_list[i];
        }
        sum += last_score
    }
    sum
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q1input.txt");

    // Pass ownership of entries through to the "get_similarity_score" function
    println!("2024 - Question 1A: Total distance between location IDs = {}", get_similarity_score(entries));
}
