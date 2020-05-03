extern crate unicode_segmentation;
use crate::unicode_segmentation::UnicodeSegmentation;
pub fn reverse(input: &str) -> String {
    if "" == input {
        String::new();
    }
    input.graphemes(true).rev().collect()
}

