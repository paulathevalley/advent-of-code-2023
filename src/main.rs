use std::fs; // Source: https://doc.rust-lang.org/stable/book/ch12-02-reading-a-file.html

// Day 1
// https://adventofcode.com/2023/day/1

// Source: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

// Source: https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
// Usage: print_type_of(&line);
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let file_path = "./data/day1.txt";

    let lines: Vec<String> = read_lines(file_path);

    let mut part1 = 0;
    // let mut part2 = 0;

    // // Consumes the iterator, returns an (Optional) String
    for line in lines {
        // line has type "alloc::string::String"
        part1 = part1 + part1_calibration_value(&line);
        // part2 = part2 + part2_calibration_value(&line);
    }
    println!("part1 {part1}");
    // println!("part2 {part2}");
    let output = part2_calibration_value("two1nine");
    println!("two1nine: {output}");
    let output2 = part2_calibration_value("eightwothree");
    println!("eightwothree: {output2}");
}

fn part1_calibration_value(line: &str) -> i32 {
    let digits: Vec<&str> = line.matches(char::is_numeric).collect();

    let first_digit = &digits[0];
    let last_digit = &digits[digits.len() - 1];

    let result: i32 = format!("{first_digit}{last_digit}").parse().unwrap();

    return result;
}

fn part2_calibration_value(line: &str) -> i32 {
    let digits: Vec<&str> = line.matches(char::is_numeric | "one").collect();

    let first_digit = &digits[0];
    let last_digit = &digits[digits.len() - 1];

    let result: i32 = format!("{first_digit}{last_digit}").parse().unwrap();

    return result;
}

#[test]
fn part1_calibration_first_line() {
    let input = "1abc2";
    let output = part1_calibration_value(&input);
    assert_eq!(output, 12);
}

#[test]
fn part1_calibration_example() {
    let lines = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let mut result = 0;

    for line in lines {
        result = result + part1_calibration_value(&line);
    }
    assert_eq!(result, 142);
}

#[test]
fn part2_calibration_example() {
    let lines = [
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];
    let mut result = 0;

    for line in lines {
        result = result + part2_calibration_value(&line);
    }
    assert_eq!(result, 281);
}

// before running a rust program, you must compile it first
// `$ rustc main.rs`
// it will generate a binary executable
// `$ ls`
// `main    main.rs`
// run the file
// `$ ./main`
