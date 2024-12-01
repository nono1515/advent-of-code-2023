use day05::line_to_nums;

fn main() {
    let input = include_str!("../../input.txt");

    let mappings = input.split("\n\n").into_iter();
    let input_num = line_to_nums(mappings.clone().next().unwrap());

    let outputs: Vec<_> = input_num
        .iter()
        .map(|n| {
            let mut input = *n;
            // skip the line of seeds
            for map in mappings.clone().skip(1) {
                // skip the description line
                for line in map.lines().skip(1) {
                    let data = line_to_nums(line);
                    if data[1] <= input && input < data[1] + data[2] {
                        input = data[0] + input - data[1];
                        break;
                    }
                }
            }
            input
        })
        .collect();

    println!("{}", outputs.iter().min().unwrap())
}


