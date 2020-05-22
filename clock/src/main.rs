use clock::*;

pub fn main() {
    Clock::new(10, 3).add_minutes(-30);
}