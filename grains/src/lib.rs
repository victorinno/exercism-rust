pub fn square(s: u32) -> u64 {
    let two: u64 = 2;
    match s {
        s if s > 64 || s < 1 => panic!("Square must be between 1 and 64"),
        s => two.pow(s - 1),
    }
}

pub fn total() -> u64 {
    (1..65).map(|x| square(x)).fold(0, |acc, x| acc + x)
}
