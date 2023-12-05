use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};




fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_4_input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines()
        .map(|r| r.expect("Failed to read line"))
        .collect();

    let numbers: Vec<(Vec<usize>, Vec<usize>)> = lines.iter()
        .map(|l| {
            let mut a = l.split(": ").nth(1).unwrap().split(" | ");
            let lhs: Vec<usize> = a.next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().expect("Invalid format"))
                .collect();
            let rhs: Vec<usize> = a.next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().expect("Invalid format"))
                .collect();

            (lhs, rhs)
        })
        .collect();

    let mut card_counts = vec![1usize; numbers.len()];

    for (i, (lhs, rhs)) in numbers.iter().enumerate() {
        let matches = rhs.iter().filter(|n| lhs.contains(n)).count();
        let num_cards = card_counts[i];

        for n in (i + 1)..=(i + matches) {
            card_counts[n] += 1 * num_cards;
        }
    }

    println!("{}", card_counts.iter().sum::<usize>());

    Ok(())
}

// part 1
// fn main() -> Result<(), Box<dyn Error>> {
//     let file = File::open("day_4_input.txt")?;
//     let reader = BufReader::new(file);
//     let lines: Vec<String> = reader.lines()
//         .map(|r| r.expect("Failed to read line"))
//         .collect();
//
//     let numbers: Vec<(Vec<usize>, Vec<usize>)> = lines.iter()
//         .map(|l| {
//             let mut a = l.split(": ").nth(1).unwrap().split(" | ");
//             let lhs: Vec<usize> = a.next()
//                 .unwrap()
//                 .split_whitespace()
//                 .map(|n| n.parse().expect("Invalid format"))
//                 .collect();
//             let rhs: Vec<usize> = a.next()
//                 .unwrap()
//                 .split_whitespace()
//                 .map(|n| n.parse().expect("Invalid format"))
//                 .collect();
//
//             (lhs, rhs)
//         })
//         .collect();
//
//     let scores: Vec<usize> = numbers.iter()
//         .map(|(lhs, rhs)| {
//             let mut score = 0usize;
//
//             for n in rhs {
//                 if lhs.contains(n) {
//                     if score == 0 {
//                         score = 1;
//                     } else {
//                         score *= 2;
//                     }
//                 }
//             }
//
//             score
//         })
//         .collect();
//
//     println!("{}", scores.iter().sum::<usize>());
//
//     Ok(())
// }
