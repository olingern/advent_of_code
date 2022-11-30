use std::{fs::File, str::Lines};

fn read_file(location: &str) -> String {
    let mut file = File::open(location).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();

    contents
}

fn calculate_total_increasing(file_lines: Lines) -> i32 {
    let mut prev_num = 0;
    let mut total_increasing = 0;
    let mut iter = 0;

    for line in file_lines.filter(|x| !x.is_empty()) {
        let num = line.parse::<i32>().unwrap();

        if iter > 0 && num > prev_num {
            total_increasing += 1;
        }

        prev_num = num;
        iter += 1;
    }

    total_increasing
}

#[test]
fn test_calculate_total_increasing_basic() {
    let test_input = "1\n\
        2\n\
        3\n\
        4";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 3);
}

#[test]
fn test_calculate_total_increasing_mixed() {
    let test_input = "0\n\
        0\n\
        1\n\
        2";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 2);
}

#[test]
fn test_calculate_total_increasing_one_line() {
    let test_input = "1\n";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 0);
}

#[test]
fn test_calculate_total_increasing_no_increase() {
    let test_input = "1\n\
    1\n\
    1\n\
    1";

    let test_lines = test_input.lines();
    let result = calculate_total_increasing(test_lines);
    assert_eq!(result, 0);
}

fn main() {
    println!(
        "{}",
        calculate_total_increasing(read_file("src/input.txt").lines())
    );
}
