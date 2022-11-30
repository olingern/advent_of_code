use std::{fs::File, str::Lines};

const WINDOW_SIZE: usize = 3;

fn read_file(location: &str) -> String {
    let mut file = File::open(location).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();

    contents
}

fn get_sum_of_range(range_str: [&str; 3]) -> u64 {
    range_str.iter().map(|x| x.parse::<u64>().unwrap()).sum()
}

fn calculate_total_increasing(file_lines: Lines) -> i32 {
    let arr: Vec<&str> = file_lines.collect();

    // ensure we can always operate on at least 3 lines
    if arr.len() < WINDOW_SIZE {
        return 0;
    }

    // loop vars
    let end = arr.len() - WINDOW_SIZE;
    let mut iter: usize = 1;

    // result vars
    let mut total_increasing = 0;
    let mut prev_sum = get_sum_of_range([arr[0], arr[1], arr[2]]);

    // slide window over array
    while iter <= end {
        let c: [&str; WINDOW_SIZE] = [arr[iter], arr[iter + 1], arr[iter + 2]];

        let curr_sum = get_sum_of_range(c);

        if curr_sum > prev_sum {
            total_increasing += 1;
            println!("increase! {}", total_increasing);
        } else {
            println!("increase!");
        }

        prev_sum = curr_sum;
        iter += 1;
    }

    total_increasing
}

#[test]
fn test_calculate_total_increasing_invalid_window() {
    let test_input = "1\n\
        2\n";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 0);
}

#[test]
fn test_calculate_total_increasing_one_window() {
    let test_input = "1\n\
        2\n\
        3\n";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 0);
}

#[test]
fn test_calculate_total_increasing_multiple_windows() {
    let test_input = "199\n\
    200\n\
    208\n\
    210\n\
    200\n\
    207\n\
    240\n\
    269\n\
    260\n\
    263\n";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 5);
}

fn main() {
    println!(
        "{}",
        calculate_total_increasing(read_file("src/input.txt").lines())
    );
}
