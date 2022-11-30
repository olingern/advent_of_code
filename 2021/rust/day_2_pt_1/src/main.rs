use std::{fs::File, str::Lines};

fn read_file(location: &str) -> String {
    let mut file = File::open(location).unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();

    contents
}

fn get_final_position(lines: Lines) -> i32 {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        let line_items = line.split(" ").collect::<Vec<_>>();

        if line_items.len() != 2 {
            panic!("Invalid line detected");
        }

        match line_items[0] {
            "forward" => x += line_items[1].parse::<i32>().unwrap(),
            "up" => y -= line_items[1].parse::<i32>().unwrap(),
            "down" => y += line_items[1].parse::<i32>().unwrap(),
            _ => panic!("Invalid direction detected"),
        }
    }

    x * y
}

#[test]
fn test_get_final_position_for_example_01() {
    let contents = read_file("src/example_01.txt");
    let lines = contents.lines();

    assert_eq!(get_final_position(lines), 150);
}

#[test]
fn test_get_final_position_for_example_02() {
    let contents = read_file("src/example_02.txt");
    let lines = contents.lines();

    assert_eq!(get_final_position(lines), 0);
}

#[test]
fn test_get_final_position_for_example_03() {
    let contents = read_file("src/example_03.txt");
    let lines = contents.lines();

    assert_eq!(get_final_position(lines), -20);
}

fn main() {
    println!(
        "{:?}",
        get_final_position(read_file("src/input.txt").lines())
    );
}
