#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let input = super::super::common::read_file("src/inputs/day_2/pt_1_example.txt");

        assert_eq!(1, super::get_answer_pt_1(&input));
    }

    #[test]
    fn test_get_rps_value() {
        assert_eq!(1, super::get_rps_value("A"));
        assert_eq!(2, super::get_rps_value("B"));
        assert_eq!(3, super::get_rps_value("C"));

        assert_eq!(1, super::get_rps_value("X"));
        assert_eq!(2, super::get_rps_value("Y"));
        assert_eq!(3, super::get_rps_value("Z"));
    }
}

pub fn get_rps_value(rps: &str) -> i32 {
    match rps {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

pub fn get_rps_winner(choice_p1: &str, choice_p2: &str) -> i32 {
    match rps {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

pub fn get_answer_pt_1(input: &String) -> i32 {
    let mut split_line: Vec<&str> = Vec::new();

    for line in input.lines() {
        split_line = line.split(" ").collect::<Vec<&str>>();
    }

    1
}
