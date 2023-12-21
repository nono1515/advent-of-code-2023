use indexmap::IndexMap;

fn main() {
    let input = b"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let input = include_bytes!("../../input.txt");

    let input = if input[input.len() - 1] == b'\n' {
        &input[0..input.len() - 1]
    } else {
        input
    };

    let mut boxes = IndexMap::new();

    for instruction in input.split(|&b| b == b',') {
        let i = instruction
            .iter()
            .position(|&b| b == b'-' || b == b'=')
            .unwrap();
        let label = &instruction[..i];
        let hash_ = hash(label);
        match instruction[i] {
            b'=' => {
                let focal: u64 = String::from_utf8(instruction[i + 1..].to_vec())
                    .unwrap()
                    .parse()
                    .unwrap();
                boxes
                    .entry(hash_)
                    .or_insert(IndexMap::new())
                    .insert(utf2str(label), focal);
            }
            b'-' => {
                if boxes.contains_key(&hash_) {
                    boxes
                        .get_mut(&hash(label))
                        .unwrap()
                        .shift_remove(&utf2str(label));
                }
            }
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    for (k, lenses) in &boxes {
        for (i, lens) in lenses.iter().enumerate() {
            sum += (k + 1) * (i as u64 + 1) * lens.1;
        }
    }

    println!("{:?}", sum);
}

fn utf2str(utf: &[u8]) -> String {
    String::from_utf8(utf.to_vec()).unwrap()
}

fn hash(s: &[u8]) -> u64 {
    s.iter()
        .filter(|&b| *b != b'\n')
        .fold(0, |acc, &b| (acc + b as u64) * 17)
        % 256
}
