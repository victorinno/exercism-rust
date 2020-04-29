fn ppdif(x: u32) -> u32{
    let mut y:u64 = x as u64;
    let  mut digit_count: u32 = 0;
    while y > 0 {
        digit_count += 1;
        y = y / 10;
    }
    let mut total: u64 = 0;
    let mut x2: u64 = x as u64;
    while x2 > 0 {
        let d: u64 = x2 % 10;
        total = total + d.pow(digit_count);
        x2 = x2 / 10;
    }
    return total as u32;
}

fn ppdif_cycle(x: u32) -> u32 {
    let mut seen: Vec<u32> = Vec::new();
    let mut x2: u32 = x;
    while !seen.contains(&x2) {
        seen.push(x2);
        x2 = ppdif(x2);
    }
    let mut cycle: Vec<u32> = Vec::new();
    while !cycle.contains(&x2) {
        cycle.push(x2);
        x2 = ppdif(x2);
    }
    return *cycle.get(0).unwrap();

}

pub fn is_armstrong_number(num: u32) -> bool {
    num == ppdif_cycle(num)
}
