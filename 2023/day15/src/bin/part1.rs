fn main() {
    // let input = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = include_bytes!("../../input.txt");

    let sum: u64 = input
        .split(|&b| b == b',')
        .map(|s| s.iter().filter(|&b| *b != b'\n').fold(0, |acc, &f| ((acc + f as u64) * 17) % 256))
        .sum();

    println!("Part 1: {}", sum);
}
