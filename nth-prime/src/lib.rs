use std::convert::TryInto;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Index{
    p: isize,
    i: isize,
    ex: isize
}

impl Index {
    /// Creates a new Viking.
    fn new(p: isize, i: isize, ex: isize) -> Index {
        Index { p: p, i: i, ex: ex }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut primes = Vec::new();
    let p: isize = 2;
    let mut indices = HashMap::new();
    while primes.len() != (n + 1).try_into().unwrap() {
        if test(p, &indices) {
            primes.push(p);
        }
    }
    return primes.pop().unwrap().try_into().unwrap();
}

fn exp_1_n(p: isize, indexes: &HashMap<&Index, isize>) -> Vec<isize> {
    let mut ex: Vec<isize> = Vec::<isize>::new();
    ex.push(1);
    let minus_one: isize = -1;
    for i in 1..p {
        if indexes.contains_key(&Index::new(p, i, ex[ex.len() - 1])) {
            ex.push(*indexes.get(&Index::new(p, i, ex[ex.len() - 1])).unwrap());
        }else {
            indexes.insert(&Index::new(p, i, ex[ex.len() - 1]), ex[ex.len() - 1] * (minus_one * (p - i)) / (i + 1));
            ex.push(*indexes.get(&Index::new(p, i, ex[ex.len() - 1])).unwrap());
        }
        
    }
    ex.reverse();
    return ex;
}

fn test(p: isize, indexes: &HashMap<&Index, isize>) -> bool {
    if p < 2 {
        return false;
    }
    let mut ex = exp_1_n(p,indexes);
    ex[0] += 1;
    let mut i = 1;
    ex.pop();
    for mult in ex {
        if (mult + i) % p == 0 {
            return false;
        }
        if i > 0 {
            i = 0;
        }
    }
    return true;
}
