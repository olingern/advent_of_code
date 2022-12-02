#[cfg(test)]
mod tests {
    #[test]
    fn get_answer_pt_1_test() {
        let input = super::super::common::read_file("src/inputs/day_1_pt_1_example.txt");

        assert_eq!(super::get_answer_pt_1(&input), 24000);
    }

    #[test]
    fn get_answer_pt_2_test() {
        let input = super::super::common::read_file("src/inputs/day_1_pt_1_example.txt");

        assert_eq!(super::get_answer_pt_2(&input), 45000);
    }
}

pub fn get_answer_pt_1(input: &String) -> i32 {
    let mut greatest_elf_calories = 0;
    let mut curr_elf_calories = 0;

    for line in input.lines() {
        if line.trim() == "" {
            if curr_elf_calories > greatest_elf_calories {
                greatest_elf_calories = curr_elf_calories;
            }
            curr_elf_calories = 0;
        } else {
            curr_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    greatest_elf_calories
}

pub fn get_answer_pt_2(input: &String) -> i32 {
    let mut curr_elf_calories = 0;
    let mut elf_calores: Vec<i32> = Vec::new();

    for line in input.lines() {
        if line.trim() == "" {
            elf_calores.push(curr_elf_calories);
            curr_elf_calories = 0;
        } else {
            curr_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    if curr_elf_calories != 0 {
        elf_calores.push(curr_elf_calories);
    }

    // sort the vector
    elf_calores.sort();

    elf_calores[(elf_calores.len() - 3)..elf_calores.len()]
        .iter()
        .sum()
}
