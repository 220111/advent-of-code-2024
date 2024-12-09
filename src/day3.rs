use std::fs;

pub fn main() {
    println!("Day 3:");
    let file_path: String = "./src/day3-test.txt".to_string();
    let content =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read file");
    println!("In file {}", file_path);

    

    println!("Total safe: {}", total);
    println!("Total safe with tolerance: {}", total_with_tolerance);
}
