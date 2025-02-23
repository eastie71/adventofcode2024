use std::fs;

/*
Question 5A) Calculate the sum of the middle page numbers from the correctly ordered updates

There is 2 sets of imput data here:
1) A list of page order rules
eg. 47|53, 97|13, 97|61, 97|47, 75|29, etc
2) A list of page order updates
eg. 75,47,61,53,29
    97,61,53,29,13
*/

// For the following 10 lines of text the word pattern 'X-MAS' appears 9 times
const TEST_ENTRIES: [&str; 28] =   ["47|53",
                                    "97|13",
                                    "97|61",
                                    "97|47",
                                    "75|29",
                                    "61|13",
                                    "75|53",
                                    "29|13",
                                    "97|29",
                                    "53|29",
                                    "61|53",
                                    "97|53",
                                    "61|29",
                                    "47|13",
                                    "75|47",
                                    "97|75",
                                    "47|61",
                                    "75|61",
                                    "47|29",
                                    "75|13",
                                    "53|13",
                                    "",
                                    "75,47,61,53,29",
                                    "97,61,53,29,13",
                                    "75,29,13",
                                    "75,97,47,61,53",
                                    "61,13,29",
                                    "97,13,75,29,47"]; 

struct PageElement {
    page_number: usize,
    page_list: Vec<usize>
}

struct PageOrderUpdateElement {
    update_list: Vec<usize>,
    correct: bool,
    corrected: bool
}

struct PageData {
    page_order_rules: Vec<PageElement>,
    page_order_updates: Vec<PageOrderUpdateElement>
}

impl PageData {
    fn new() -> Self {
        PageData { page_order_rules: Vec::new(), page_order_updates: Vec::new() }
    }

    fn set_page_order_rules(&mut self, page_order_rules: Vec<PageElement>) {
        self.page_order_rules = page_order_rules;
    }

    fn set_page_order_updates(&mut self, page_order_updates: Vec<PageOrderUpdateElement>) {
        self.page_order_updates = page_order_updates;
    }

    fn display(&self) {
        println!("Page Order Rules:");
        for page_element in &self.page_order_rules {
            print!("Page Number: {}, List: ", page_element.page_number);
            for page_list_number in &page_element.page_list {
                print!("{} ", page_list_number);
            }
            println!();
        }

        println!("Page Order Updates: ");
        for page_order_update in &self.page_order_updates {
            for page_number in &page_order_update.update_list {
                print!("{} ", page_number);
            }
            println!(" - Correct: {}", page_order_update.correct);
        }
    }

    fn display_corrected(&self) {
        println!("Page Order Updates: ");
        for page_order_update in &self.page_order_updates {
            if page_order_update.corrected {
                for page_number in &page_order_update.update_list {
                    print!("{} ", page_number);
                }
                println!(" - Correct: {}, Corrected: {}", page_order_update.correct, page_order_update.corrected);
            }
        }
    }

    fn display_incorrect(&self) {
        println!("INCORRECT Page Order Updates: ");
        for page_order_update in &self.page_order_updates {
            if !page_order_update.correct {
                for page_number in &page_order_update.update_list {
                    print!("{} ", page_number);
                }
                println!(" - Correct: {}, Corrected: {}", page_order_update.correct, page_order_update.corrected);
            }
        }
    }

    fn set_correct_page_order_updates(&mut self) {
        for page_order_update in &mut self.page_order_updates {
            let mut correct = true;
            let mut position = 0;

            //println!("Page Order Update: {}", count);

            for page_number in &page_order_update.update_list {
                // If we have reached the last number in the update list, then mark as correct and break
                if position == page_order_update.update_list.len()-1 {
                    break;
                }
                // Find the valid page_list element for the page_number in the update list, and if its not found, the set correct to false and break
                let valid_page_element = self.page_order_rules.iter().find(|&x| x.page_number == *page_number); //returns Option<&PageElement>
                if valid_page_element.is_none() {
                    //println!("No Page Order Rules found for Page Number {}", page_number);
                    correct = false;
                    break;
                }
                position += 1;
                // Now check the remaining page_numbers in the update list to see if they exist in the page_list
                for i in position..page_order_update.update_list.len()-1 {
                    let page_num = page_order_update.update_list[i];
                    // Does the page_number exist in the page_list?
                    let found_page_number = valid_page_element.unwrap().page_list.iter().find(|&&x| x == page_num);
                    // If not found, then set correct to false and break
                    if found_page_number.is_none() {
                        //println!("Update Page Number {} not found in Page List for Page Number {}", page_num, page_number);
                        correct = false;
                        break;
                    }
                }
                if !correct {
                    break;
                }
            }
            page_order_update.correct = correct;
        }
    }

    // Perform a bubble sort on the update list, to ensure the page numbers are in the correct order
    fn sort_updates(arr: &mut Vec<usize>, rules: &Vec<PageElement>) {
        let len = arr.len();
        
        if len <= 1 {
            return;
        }
    
        for i in 0..len {
            let mut swapped = false;
            for j in 0..len - 1 - i {
                let element1_rules = rules.iter().find(|&x| x.page_number == arr[j]); // Returns Option<&PageElement>
                let mut found_page_number = None;
                if !element1_rules.is_none() {
                    // Because we know element1_rules is not None, we can safely unwrap it
                    // Because element1_rules is a reference to PageElement, and find returns a reference to the found element, we need to double dereference it
                    found_page_number = element1_rules.unwrap().page_list.iter().find(|&&x| x == arr[j+1]);
                }
                if found_page_number.is_none() {
                    arr.swap(j, j + 1);
                    swapped = true;
                }
            }
            
            if !swapped {
                break;
            }
        }
    }

    fn fix_incorrect_page_order_updates(&mut self) {
        while self.page_order_updates.iter().any(|x| x.corrected == false && x.correct == false) {
            for page_order_update in &mut self.page_order_updates {
                if !page_order_update.correct {
                    Self::sort_updates(&mut page_order_update.update_list, &self.page_order_rules);
                    page_order_update.corrected = true; // Set to true, as we have now corrected the update list
                }
            }
        }
    }

    // fn fix_incorrect_page_order_updates(&mut self) {
    //     while self.page_order_updates.iter().any(|x| x.corrected == false && x.correct == false) {
    //         for page_order_update in &mut self.page_order_updates {
    //             if !page_order_update.correct {
    //                 let mut correct = true;
    //                 //let mut position = 0;
    //                 let update_list_len = page_order_update.update_list.len();
    //                 for position in 0..update_list_len {
    //                     let page_number = page_order_update.update_list[position];
    //                     // If we have reached the last number in the update list, then finished processing
    //                     if position == update_list_len-1 {
    //                         break;
    //                     }
    //                     // Find the valid page_list element for the page_number in the update list, and if its not found, try to correct it and break
    //                     let valid_page_element = self.page_order_rules.iter().find(|&x| x.page_number == page_number);
    //                     if valid_page_element.is_none() {
    //                         // Move this page_number to the end of the list
    //                         page_order_update.update_list.remove(position);
    //                         page_order_update.update_list.push(page_number);
    //                         // Reset the position to the start of the list
    //                         //println!("No Page Order Rules found for Page Number {} - Push it to the End", page_number);
    //                         correct = false;
    //                         break;
    //                     }
    //                     // Now check the remaining page_numbers in the update list to see if they exist in the page_list
    //                     for i in position+1..update_list_len-1 {
    //                         let page_num = page_order_update.update_list[i];
    //                         // Does the page_number exist in the page_list?
    //                         let found_page_number = valid_page_element.unwrap().page_list.iter().find(|&x| x == &page_num);
    //                         // If not found, then set correct to false and break
    //                         if found_page_number.is_none() {
    //                             //println!("Update Page Number {} not found in Page List for Page Number {}", page_num, page_number);
    //                             // Move this page number back 1 place in the list
    //                             page_order_update.update_list.remove(i);
    //                             page_order_update.update_list.insert(i-1, page_num);
    //                             correct = false;
    //                             break;
    //                         }
    //                     }
    //                     if !correct {
    //                         break;
    //                     }
    //                 }
    //                 page_order_update.corrected = correct;
    //             }
    //         }
    //     }
    // }

    fn sum_middle_page_numbers_from_corrected_updates(&self) -> usize {
        let mut total_middle_page_numbers = 0;
        for page_order_update in &self.page_order_updates {
            if page_order_update.corrected {
                let middle_position = page_order_update.update_list.len() / 2;
                let middle_page_number = page_order_update.update_list[middle_position];
                total_middle_page_numbers += middle_page_number;
            }
        }
        total_middle_page_numbers
    }
}

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

fn get_total_middle_number_from_corrected_updates(input_data: Vec<String>) -> usize {
    let mut page_data = PageData::new();
    let mut page_order_rules: Vec<PageElement> = Vec::new();
    let mut page_order_updates: Vec<PageOrderUpdateElement> = Vec::new();

    let mut is_page_order_rule = true;
    for line in input_data {
        // If line is empty, then we are no longer processing page order rules, its the page order updates section
        if line.is_empty() {
            is_page_order_rule = false;
            continue;
        }

        if is_page_order_rule {
            let page_order_rule_parts: Vec<&str> = line.split("|").collect();
            let page_number = page_order_rule_parts[0].parse::<usize>().unwrap();
            let page_list_number = page_order_rule_parts[1].parse::<usize>().unwrap();

            // If page_number already exists in page_order_rules, then add page_list_number to the existing page_list
            if let Some(page_element) = page_order_rules.iter_mut().find(|x| x.page_number == page_number) {
                // page_element is mutable, so we can add to the page_list
                page_element.page_list.push(page_list_number);
            } else {
                // Otherwise, create a new PageElement and add it to page_order_rules
                let page_list: Vec<usize> = vec![page_list_number];
                page_order_rules.push(PageElement { page_number, page_list });
            }
        } else {
            let update_list: Vec<usize> = line.split(",").map(|s|s.parse::<usize>().unwrap()).collect();
            //let correct = false;
            //let corrected = false;
            page_order_updates.push(PageOrderUpdateElement { update_list, correct:false, corrected:false });
        }
    }

    // Sort each page_list in page_order_rules
    for page_element in &mut page_order_rules {
        page_element.page_list.sort();
    }
    // Sort each page element in page_order_rules by page_number
    page_order_rules.sort_by(|a, b| a.page_number.cmp(&b.page_number));

    page_data.set_page_order_rules(page_order_rules);
    page_data.set_page_order_updates(page_order_updates);

    //page_data.display();

    page_data.set_correct_page_order_updates();

    page_data.fix_incorrect_page_order_updates();

    page_data.display_corrected();

    page_data.sum_middle_page_numbers_from_corrected_updates()
}

fn main() {
    // Convert TEST_ENTRIES from Vec<&str> to Vec<String>
    let _test_entries_strings: Vec<String> = TEST_ENTRIES.into_iter().map(|s|String::from(s)).collect();

    let entries = get_entries("Q5input.txt");

    // Pass ownership of entries through to the "get_total_middle_number_from_updates" function
    println!("2024 - Question 5B: The sum of the middle page numbers from the corrected order updates = {}", get_total_middle_number_from_corrected_updates(entries));
}
