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
    let mut aim = 0;

    for line in lines {
        let line_items = line.split(" ").collect::<Vec<_>>();

        if line_items.len() != 2 {
            panic!("Invalid line detected");
        }

        let parsed_value = line_items[1].parse::<i32>().unwrap();

        match line_items[0] {
            "forward" => {
                x += parsed_value;
                y += aim * parsed_value; // forward increases your depth by your aim multiplied by X.
            }
            "up" => {
                aim -= parsed_value; // up X decreases your aim by X units.
            }
            "down" => {
                aim += parsed_value; // down X increases your aim by X units.
            }
            _ => panic!("Invalid direction detected"),
        }
    }

    x * y
}

#[test]
fn test_get_final_position_for_example_01() {
    let contents = read_file("src/example_01.txt");
    let lines = contents.lines();

    assert_eq!(get_final_position(lines), 900);
}

fn main() {
    println!(
        "{:?}",
        get_final_position(read_file("src/input.txt").lines())
    );
}
