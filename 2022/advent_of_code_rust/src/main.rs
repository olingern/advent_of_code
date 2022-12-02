mod advent_of_code;

use advent_of_code::{common, day_1};

fn main() {
    common::print_banner();

    let day1 = common::read_file("src/inputs/day_1_pt_1_real.txt");

    println!("Day 1 pt 1 answer: {:?}", day_1::get_answer_pt_1(&day1));
    println!("Day 1 pt 2 answer: {:?}", day_1::get_answer_pt_2(&day1));
}
