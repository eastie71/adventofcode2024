use std::fs;

/*
Question 2B) Calculate the total safe reports (after applying tolerance). Each line is considered a report.
To calculate if a line is safe, all levels must be increasing or decreasing AND any 2 adjacent levels must be between 1 and 3 difference.
HOWEVER - can now tolerate a single bad level - ie. if ONE number is removed - check again if it complies with the safe rules.
*/

// For the following 6 report lines
const TEST_ENTRIES: [&str; 6] =    ["7 6 4 2 1",  // SAFE - all decreasing
                                    "1 2 7 8 9",  // UNSAFE - gap b/w 2 and 7 is > 3 AND removing either 2 or 7 does not help.
                                    "9 7 6 2 1",  // UNSAFE - gap b/w 6 and 2 is > 3 AND removing either 6 or 2 does not help.
                                    "1 3 2 4 5",  // SAFE - level increases and then decreases, HOWEVER removing either 3 or 2 helps.
                                    "8 6 4 4 1",  // SAFE - no difference b/w 4 and 4, HOWEVER removing either 4 makes it safe.
                                    "1 3 6 7 9"]; // SAFE - all increasing

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

fn is_safe_report(results: &Vec<&str>) -> bool {
    //println!("Report results: {:#?}", results);

    let mut first_num:  i32 = results[0].trim().parse().unwrap();
    let mut second_num: i32 = results[1].trim().parse().unwrap();
    //println!("nums: {},{}", first_num, second_num);
    let mut diff = second_num - first_num;
    let mut increase = false;
    if diff == 0 || diff.abs() > 3 {
        return false;
    } else if diff > 0 {
        increase = true;
    }

    for i in 1..results.len()-1 {
        first_num = results[i].trim().parse().unwrap();
        second_num = results[i+1].trim().parse().unwrap();
        diff = second_num - first_num;
        if diff == 0 || diff.abs() > 3 {
            return false;
        }
        if diff > 0 && increase == false {
            return false;
        }
        if diff < 0 && increase == true {
            return false
        }
    }
    true
}

fn is_safe_report_with_tolerance(results: &Vec<&str>) -> bool {
    if is_safe_report(results) {
        return true;
    }
    // Traverse through the result line, removing values to see if this makes the report safe.
    for i in 0..results.len() {
        // Make a copy of this results line
        let mut adjusted_results = results.clone();
        // Remove a result to see if the report is safe now?
        adjusted_results.remove(i);
        // If the report is safe now, just return true, else try remove the next result value...
        if is_safe_report(&adjusted_results) {
            return true;
        }
    }
    false
}

fn get_num_of_safe_reports(reports: Vec<String>) -> i32 {
    let mut sum = 0;
    
    for report in reports {
        let report_results = report.split(" ").collect::<Vec<&str>>();
        if is_safe_report_with_tolerance(&report_results) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q2input.txt");

    // Pass ownership of entries through to the "get_num_of_safe_reports" function
    println!("2024 - Question 2B: Total number of safe reports (after tolerance applied) = {}", get_num_of_safe_reports(entries));
}
