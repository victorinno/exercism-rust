pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let r: u32 = (1..limit)
        .filter(|x| {
            factors.iter().fold(false, |acc, z| {
                acc || match z {
                    z if *z == 0 => false,
                    z => x % z == 0,
                }
            })
        })
        .fold(0, |acc, x| acc + x);
    r
}
