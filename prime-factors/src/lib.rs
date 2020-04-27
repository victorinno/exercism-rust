pub fn factors(n: u64) -> Vec<u64> {
    let mut dividend: u64 = n;
    let mut factors: Vec<u64> = Vec::new();
    let mut prime_divisor: u64 = 2;
    while dividend > 1 {
        if dividend % prime_divisor == 0 {
            factors.push(prime_divisor);
            dividend /= prime_divisor;
        } else {
            prime_divisor += 1;
        }
    }
    factors
}
