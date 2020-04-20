pub fn raindrops(n: u32) -> String {
    let mut r = "".to_string();

    if n % 3 == 0 {
        r.push_str("Pling");
    }
    if n % 5 == 0 {
        r.push_str("Plang");
    }
    if n % 7 == 0 {
        r.push_str("Plong");
    }
    if r == "" {
        r.push_str(&n.to_string());
    }

    r
}
