mod values_list;
use std::io;

fn main() {
    let values = values_list::get_values();
    println!("Help us sort the values by importance to you.");
    let sorted_values = decision_tree_sort(&values);
    println!("Values sorted by your preferences: {:?}", sorted_values);
}

fn decision_tree_sort<'a>(values: &'a [&'a str]) -> Vec<&'a str> {
    let mut sorted_values = vec![];
    for &value in values {
        if sorted_values.is_empty() {
            sorted_values.push(value);
        } else {
            let pos = find_position(value, &sorted_values);
            sorted_values.insert(pos, value);
        }
    }
    sorted_values
}

fn find_position(value: &str, sorted_values: &[&str]) -> usize {
    let mut low = 0;
    let mut high = sorted_values.len();

    while low < high {
        let mid = (low + high) / 2;
        if ask_user(value, sorted_values[mid]) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn ask_user(new_value: &str, existing_value: &str) -> bool {
    println!(
        "Which is more important to you? Type 1 for '{}', or 2 for '{}'",
        new_value, existing_value
    );
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim() == "1"
}
