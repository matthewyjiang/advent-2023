// day 1: trebuchet

/*
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
 */

use std::cmp;
use std::collections::HashMap;
use std::fs;

fn digit_from_left(digits: &HashMap<&str, i32>, s: &str) -> i32 {
    for i in 0..s.len() {
        let window = &s[0..s.len() - i];
        if digits.contains_key(window) {
            return digits[window];
        }
    }
    return 0;
}

fn digit_from_right(digits: &HashMap<&str, i32>, s: &str) -> i32 {
    for i in 0..s.len() {
        let window = &s[i..s.len()];
        if digits.contains_key(window) {
            return digits[window];
        }
    }
    return 0;
}

fn main() {
    let mut digits = HashMap::new();
    digits.insert("one", 1);
    digits.insert("two", 2);
    digits.insert("three", 3);
    digits.insert("four", 4);
    digits.insert("five", 5);
    digits.insert("six", 6);
    digits.insert("seven", 7);
    digits.insert("eight", 8);
    digits.insert("nine", 9);
    digits.insert("1", 1);
    digits.insert("2", 2);
    digits.insert("3", 3);
    digits.insert("4", 4);
    digits.insert("5", 5);
    digits.insert("6", 6);
    digits.insert("7", 7);
    digits.insert("8", 8);
    digits.insert("9", 9);

    // read file input.txt
    let mut total = 0;
    // for each line
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = input.lines();
    for (_i, line) in lines.enumerate() {
        let mut answer = 0;

        for i in 0..line.len() + 1 {
            let window = &line[i..i + cmp::min(5, line.len() - i)];
            let digit = digit_from_left(&digits, &window);
            if digit != 0 {
                answer += 10 * digit;
                break;
            }
        }

        for i in (0..line.len() + 1).rev() {
            let window = &line[i - cmp::min(5, i)..i];
            let digit = digit_from_right(&digits, &window);
            if digit != 0 {
                answer += digit;
                break;
            }
        }
        total += answer;
    }

    println!("total: {}", total);
}
