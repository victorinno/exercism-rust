pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let frases = (0..list.len()-1)
        .map(|n| (list[n], list[n+1]))
        .map(|x| format!("For want of a {} the {} was lost.", x.0, x.1))
        .collect::<Vec<_>>()
        .join("\n");

    match frases {
        frases if frases == "" => format!("And all for the want of a {}.",list[0]),
        frases => format!("{}\nAnd all for the want of a {}.", frases, list[0]) 
    }
    
}
