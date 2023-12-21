fn main() {
    let input = include_str!("../../input.txt");

    let win_num_i = input.lines().next().unwrap().find(':').unwrap();
    let my_num_i = input.lines().next().unwrap().find('|').unwrap();
    let mut copies = [1; 256];

    let total: u32 = input
        .lines()
        .enumerate()
        .map(|(i, game)| {
            let win_nums = &game[win_num_i + 1..my_num_i]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let win_count: u32 = game[my_num_i + 1..]
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .filter(|num| win_nums.contains(num))
                .count() as u32;
            for ii in i + 1..=i + win_count as usize {
                copies[ii] += copies[i];
            }
            copies[i]
        })
        .sum();

    println!("{}", total);
}
