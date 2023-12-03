use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_adjacent_numbers(chars: &Vec<char>, x: usize, y: usize, length: usize, height: usize) -> Option<(usize, usize)> {
    let start_x = x.saturating_sub(1);
    let start_y = y.saturating_sub(1);
    let end_x = cmp::min(start_x.saturating_add(3), length);
    let end_y = cmp::min(start_y.saturating_add(3), height);
    let mut numbers: Vec<usize> = Vec::new();

    for y in start_y..end_y {
        let mut x_iter = start_x..end_x;


        while let Some(x) = x_iter.next() {
            let c = chars[y * length + x];

            if c.is_numeric() {
                let mut num_start = x;
                let mut num_end = 0;
                let mut num_chars: Vec<char> = Vec::new();

                // walk to the most left digit
                for dx in (0..x).rev() {
                    if !chars[y * length + dx].is_numeric() {
                        break;
                    }

                    num_start = dx;
                }

                for dx in num_start..length {
                    let c = chars[y * length + dx];

                    if !c.is_numeric() {
                        break;
                    }

                    num_chars.push(c);
                    num_end = dx;
                }

                let num_str: String = num_chars.iter().collect();
                let num: usize = num_str.parse().expect("Invalid number");

                numbers.push(num);

                if num_end >= end_x {
                    break;
                } else { // skip n of x to not find the same number again
                    x_iter.nth(num_end - x);
                }
            }
        }
    }

    if numbers.len() == 2 {
        return Some((numbers[0], numbers[1]));
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_3_input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();
    let length = lines.first().expect("Invalid file").len();
    let height = lines.len();

    let chars: Vec<char> = lines.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .flatten()
        .collect();

    let mut result = 0usize;

    for y in 0..height {
        for x in 0..length {
            let c = chars[y * length + x];

            if c != '*' {
                continue;
            }

            if let Some((num1, num2)) = get_adjacent_numbers(&chars, x, y, length, height) {
                result += num1 * num2;
            }
        }
    }

    println!("{}", result);


    Ok(())
}

// part 1
// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("day_3_input.txt")?;
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines()
//         .map(|r| r.expect("Failed to read line"))
//         .collect();
//     let length = lines.first().expect("Invalid file").len();
//     let height = lines.len();
//
//     let chars: Vec<char> = lines.iter()
//         .map(|s| s.chars().collect::<Vec<char>>())
//         .flatten()
//         .collect();
//
//     let mut is_walking = false;
//     let mut walking_digits = Vec::<(usize, char)>::new();
//     let mut digits = Vec::<usize>::new();
//
//     for (i, c) in chars.iter().enumerate() {
//         if c.is_numeric() {
//             is_walking = true;
//             walking_digits.push((i, *c));
//         } else if is_walking {
//             let mut is_part_number = false;
//             is_walking = false; // done walking
//
//             // check if there's any symbols around it other than .
//             for (digit_i, _) in &walking_digits {
//                 // check top
//                 if let (index, false) = digit_i.overflowing_sub(length) {
//                     if chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check bottom
//                 if let (index, false) = digit_i.overflowing_add(length) {
//                     if index < chars.len() && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check left
//                 if let (index, false) = digit_i.overflowing_sub(1) {
//                     if index % length != length - 1 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check right
//                 if let (index, false) = digit_i.overflowing_add(1) {
//                     if index < chars.len() && index % length != 0 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check top left
//                 if let (index, false) = digit_i.overflowing_sub(length + 1) {
//                     if (index) % length != length - 1 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check top right
//                 if let (index, false) = digit_i.overflowing_sub(length - 1) {
//                     if index % length != 0 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check bottom left
//                 if let (index, false) = digit_i.overflowing_add(length - 1) {
//                     if index < chars.len() && index % length != length - 1 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//
//                 // check bottom right
//                 if let (index, false) = digit_i.overflowing_add(length + 1) {
//                     if index < chars.len() && index % length != 0 && chars[index] != '.' && !chars[index].is_numeric() {
//                         is_part_number = true;
//                         break;
//                     }
//                 }
//             }
//
//             if is_part_number {
//                 let num_str: String = walking_digits.iter()
//                     .map(|(i, c)| c)
//                     .collect();
//
//                 let num: usize = num_str.parse()?;
//
//                 digits.push(num);
//             }
//             walking_digits.clear();
//         }
//     }
//
//     println!("{}", digits.iter().sum::<usize>());
//
//     Ok(())
// }
