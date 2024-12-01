use primitive_types::U256;

pub fn mask_of_1(u: usize) -> U256 {
    (U256::one() << u) - 1
}

pub fn u256_to_bin_string(u: U256) -> String {
    format!(
        "0b{:064b}{:064b}{:064b}{:064b}",
        u.0[3], u.0[2], u.0[1], u.0[0]
    )
}
