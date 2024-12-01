fn main() {
    let input = include_str!("../../input.txt");

    let win_num_i = input.lines().next().unwrap().find(':').unwrap();
    let my_num_i = input.lines().next().unwrap().find('|').unwrap();

    let total: u32 = input
        .lines()
        .map(|game| {
            let win_nums = &game[win_num_i + 1..my_num_i]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let win_count: u32 = game[my_num_i + 1..]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .filter(|num| win_nums.contains(num))
                .count() as u32;
            2u32.pow(win_count) >> 1
        })
        .sum();

    println!("{}", total);
}
