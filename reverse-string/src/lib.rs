
pub fn reverse(input: &str) -> String {
    if "" == input {
        String::new();
    }
    input.chars().rev().collect()
}
// I used this one to try a better performance
// extern crate unicode_segmentation;
// use unicode_segmentation::UnicodeSegmentation;
// pub fn reverse2(str_in: &str) -> String {
//     str_in
//         .graphemes(true)
//         .rev()
//         .flat_map(|c| c.chars())
//         .collect()
// }
