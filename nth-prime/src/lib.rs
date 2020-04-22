pub fn is_prime(n: u32) -> bool {
    ! (2..n).any(|i| n % i == 0)
}

pub fn nth(n: u32) -> u32 {
    let r = match n {
        n if n <= 0 => Some(2),
        n => (1..).filter(|c| is_prime(*c)).nth((n as usize) + 1),
    };
    return r.unwrap();
}

