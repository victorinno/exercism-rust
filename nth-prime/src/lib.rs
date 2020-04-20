pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let mut p = 2;
    while primes.len().into() != n+1 {
        if test(p) {
            primes.push(p);
        }
    }
    return primes.pop().unwrap();
}

fn exp_1_n(p: u32) -> Vec<u64> {
    let mut ex = Vec::new();
    ex.push(1);
    for i in 1..p {
        ex.push(ex[ex.len()-1] * (-1 * (p - i)) / (i+1));
    }
    ex.reverse();
}

fn test(p: u32) -> bool {
    if p < 2 {
        return false;
    }
    let mut ex = exp_1_n(p);
    ex[0] += 1;
    let mut i = 1;
    ex.pop();
    for mult in ex {
        if (mult + i) % p {
            return false;
        }
        if i > 0 {
            i = 0;
        }
    }
    return true;
}