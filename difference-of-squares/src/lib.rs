pub fn square_of_sum(n: u32) -> u32 {
    let result: u32 = (1..n+1).fold(0, |acc, x| acc + x).pow(2);
    return result;
}

pub fn sum_of_squares(n: u32) -> u32 {
    let result: u32 = (1..n+1).map(|x| x.pow(2)).fold(0, |acc, x| acc + x);
    return result;
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
