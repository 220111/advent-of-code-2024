use std::collections::HashMap;
use std::fs;

pub fn main() {
    println!("Day 1:");
    let file_path: String = "./src/day1-input.txt".to_string();
    let content =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read file");
    println!("In file {}", file_path);

    let mut row_a: Vec<i32> = Vec::new();
    let mut row_b: Vec<i32> = Vec::new();

    for line in content.lines() {
        let mut nums = line.split("   ");
        let num_a: i32 = nums.next().unwrap().parse().unwrap();
        let num_b: i32 = nums.next().unwrap().parse().unwrap();
        row_a.push(num_a);
        row_b.push(num_b);
    }

    row_a.sort();
    row_b.sort();

    let distances: Vec<i32> = row_a
        .iter()
        .zip(row_b)
        .map(|(a, b)| (a - b).abs())
        .collect();

    let total: i32 = distances.iter().sum();

    println!("Total Distance: {}", total);

    println!("Part 2:");
    let content =
        fs::read_to_string(file_path.clone()).expect("Should have been able to read file");
    println!("In file {}", file_path);

    let mut row_a: Vec<i32> = Vec::new();
    let mut row_b: Vec<i32> = Vec::new();

    for line in content.lines() {
        let mut nums = line.split("   ");
        let num_a: i32 = nums.next().unwrap().parse().unwrap();
        let num_b: i32 = nums.next().unwrap().parse().unwrap();
        row_a.push(num_a);
        row_b.push(num_b);
    }

    row_a.sort();
    row_b.sort();

    let mut row_a_count = HashMap::new();
    let mut row_b_count = HashMap::new();

    for i in row_a.iter() {
        row_a_count.insert(i, row_a_count.get(i).unwrap_or(&0) + 1);
    }

    for i in row_b.iter() {
        row_b_count.insert(i, row_b_count.get(i).unwrap_or(&0) + 1);
    }

    let total: i32 = row_a_count
        .keys()
        .map(|x| *x * row_a_count.get(x).unwrap() * row_b_count.get(x).unwrap_or(&0))
        .sum();

    println!("Total = {}", total);
}
