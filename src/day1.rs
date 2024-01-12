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
                // concatenate all digits in vector
                if calibration.len() == i + 1 {
                    let current = &calibration[i];
                    calibration[i] = format!("{current}{c}");
                } else {
                    calibration.push(String::from(c));
                }
            }
        }
        let mut current = &calibration[i];
        let len = calibration[i].len();
        if len == 1 {
            // duplicate the first digit so there are 2 digits
            // Source: rust compiler said to add `.to_owned()`
            calibration[i] = (current.to_owned() + current).to_string();
            current = &calibration[i];
        }
        println!("callibration: {current}");
        if len > 2 {
            // grab the 1st char & last char
            // for c in current.chars() {
            //     println!("hello {c}")
            // }
        }
        // for x in &v {
        //     println!("{}", v[x]);
        // }

        // The following errors: `String` cannot be indexed by `usize`
        // Source: https://doc.rust-lang.org/stable/book/ch08-02-strings.html?highlight=String#indexing-into-strings
        // for n in 1..len {
        //     println!("{}", line[n]);
        // }
        i = i + 1;
    }
}

// before running a rust program, you must compile it first
// `$ rustc main.rs`
// it will generate a binary executable
// `$ ls`
// `main    main.rs`
// run the file
// `$ ./main`
