fn main() {
    let input = aoc_2023::utils::get_input("day_2");
    t_1(&input);
    t_2(&input);
}

fn t_1(input: &str) {
    let sum = aoc_2023::day_2::task_1(input);
    println!("task 1: {sum}");
}

fn t_2(input: &str) {
    let sum = aoc_2023::day_2::task_2(input);
    println!("task 2: {sum}");
}
