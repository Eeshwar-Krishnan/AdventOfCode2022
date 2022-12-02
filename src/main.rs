use std::fs::File;

mod Day1;
mod Day2;
fn main() {
    Day2::problem2(File::open("src/day2/input_real.txt").unwrap());
}
