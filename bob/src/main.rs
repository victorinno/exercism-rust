use regex::Regex;
pub fn main(){
    print!("{}",Regex::new(r"[a-zA-Z]").unwrap().is_match(&":) ?"));
}