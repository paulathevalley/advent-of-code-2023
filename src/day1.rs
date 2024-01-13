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

    let lines = read_lines(file_path);

    let mut calibration: Vec<String> = vec![];
    let mut i = 0;

    // Consumes the iterator, returns an (Optional) String
    for line in lines {
        // println!("line: {i}");
        // line has type "alloc::string::String"

        // Source: https://doc.rust-lang.org/stable/book/ch08-02-strings.html
        for c in line.chars() {
            // Source: https://doc.rust-lang.org/std/primitive.char.html#method.is_digit
            if c.is_digit(10) {
                // let mut digits = vec![];
                // digits.push(c);
                // can we do everything in here? unsure.

                // concatenate all digits in vector
                if calibration.len() == i + 1 {
                    // if there is already an entry for this calibration, update it
                    let current = &calibration[i];
                    calibration[i] = format!("{current}{c}");
                } else {
                    // otherwise add new entry
                    calibration.push(String::from(c));
                }
            }
            // do we know when we've reached the last char in line?
        }

        let len = calibration[i].len();
        if len == 1 {
            let current = &calibration[i];
            // duplicate the first digit so there are 2 digits
            // Source: rust compiler said to add `.to_owned()` if current is mutable
            // calibration[i] = (current.to_owned() + current).to_string();
            calibration[i] = format!("{current}{current}");
        }
        if len > 2 {
            let current = &calibration[i];
            let mut letters = vec![];
            // grab the 1st char & last char
            for c in current.chars() {
                letters.push(c.to_string());
            }
            let first = &letters[0];
            let last = &letters[letters.len() - 1];
            let combo = first.to_owned() + last;
            calibration[i] = combo;
        }

        // The following errors: `String` cannot be indexed by `usize`
        // Source: https://doc.rust-lang.org/stable/book/ch08-02-strings.html?highlight=String#indexing-into-strings
        // for n in 1..len {
        //     println!("{}", line[n]);
        // }
        i = i + 1;
    }

    let len = calibration.len();
    let mut total: u32 = 0;
    for n in 1..len {
        let digit: &u32 = &calibration[n].parse().unwrap();
        total = total + digit;
    }
    println!("answer: {total}");
}

// before running a rust program, you must compile it first
// `$ rustc main.rs`
// it will generate a binary executable
// `$ ls`
// `main    main.rs`
// run the file
// `$ ./main`
