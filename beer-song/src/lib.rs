fn use_s(n: u32) -> String {
    match n {
        n if n > 1 => "s".to_string(),
        _n => "".to_string(),
    }
}

fn put_line_break(n: u32) -> String {
    match n {
        n if n > 0 => "\n".to_string(),
        _n => "".to_string(),
    }
}

fn verse_1(n: u32) -> String {
    match n {
        n if n == 0 => "No more bottles of beer on the wall, no more bottles of beer.".to_string(),
        n => format!(
            "{qtd} bottle{s} of beer on the wall, {qtd} bottle{s} of beer.",
            qtd = n,
            s = use_s(n)
        ),
    }
}

fn verse_2(n: u32) -> String {
    match n {
        n if n == 0 => {
            "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string()
        }
        n if n == 1 => {
            "Take it down and pass it around, no more bottles of beer on the wall.".to_string()
        }
        n => format!(
            "Take one down and pass it around, {qtd} bottle{s} of beer on the wall.",
            qtd = n - 1,
            s = use_s(n - 1)
        ),
    }
}

pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", verse_1(n), verse_2(n)).to_string()
}

pub fn sing(start: u32, end: u32) -> String {
    let mut counter: u32 = start;
    let mut song: String = String::new();
    let mut done = true;
    while done {
        song = format!(
            "{}{}{}",
            song,
            verse(counter),
            put_line_break(counter - end)
        );
        if counter == 0 || counter == end {
            done = false;
        } else {
            counter -= 1;
        }
    }
    return song;
}
