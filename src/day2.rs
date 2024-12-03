use std::fs;

pub fn main() {
    println!("Day 2:");
    let file_path: String = "./src/day2-test.txt".to_string();
    let content =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read file");
    println!("In file {}", file_path);

    let mut total: u32 = 0;
    let mut total_with_tolerance: u32 = 0;

    for line in content.lines() {
        let nums: Vec<i32> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        let inc: bool = nums.first() < nums.get(1);
        let mut nums_iter = nums.iter().peekable();
        let mut errors: u32 = 0;
        while let Some(num) = nums_iter.next() {
            let next = *match nums_iter.peek() {
                Some(x) => x,
                None => break,
            };
            if inc && next < num
                || !inc && next > num
                || (next - num).abs() > 3
                || (next - num).abs() < 1
            {
                errors += 1;
            }
        }
        if errors == 0 {
            total += 1;
        }
        if errors < 2 {
            total_with_tolerance += 1;
        }
    }

    println!("Total safe: {}", total);
    println!("Total safe with tolerance: {}", total_with_tolerance);
}
