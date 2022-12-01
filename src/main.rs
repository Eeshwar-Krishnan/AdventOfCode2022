use std::fs::File;

mod Day1;

fn main() {
    Day1::problem2(File::open("src/day1/input_test.txt").unwrap());
}
