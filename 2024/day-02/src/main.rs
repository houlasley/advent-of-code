use std::fs::File;
use std::io::{self, BufRead};
fn main() -> io::Result<()> {
    let path = "input.txt";

    // Open the file
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut result = reader
        .lines()
        .map(|line| {
            line.unwrap_or_default()
                .split_whitespace()
                .map(|num| num.parse::<i32>())
                .filter_map(Result::ok)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    println!("{:?}", result);
    fn rule_one(row: &Vec<i32>) -> Vec<(usize, i32)> {
        let mut increasing: i32 = 0;
        let mut decreasing: i32 = 0;

        let mut issues: Vec<(usize, i32)> = Vec::new();
        for (index, _number) in row.into_iter().enumerate() {
            if index == 0 {
                continue;
            } else {
                let diff = row[index] - row[index - 1];
                if diff > 0 {
                    if diff <= 3 {
                        increasing += diff;
                    } else {
                        issues.push((index, 0));
                    }
                } else if diff < 0 {
                    if diff >= -3 {
                        decreasing += diff;
                    } else {
                        issues.push((index, 0));
                    }
                } else {
                    issues.push((index, 0));
                }
            };
            if increasing > 0 && decreasing < 0 {
                issues.push((index - 1, 0));
            }
        }
        return issues;
    }
    let mut sum = 0;
    for row in result.iter_mut() {
        println!("row==={:?}", row);
        let res = rule_one(row);
        println!("res==={:?}", res);
        if res.is_empty() {
            println!("row success==={:?}", row);
            sum += 1;
            continue;
        } else {
            for something_else in row.iter().enumerate() {
                let removed = something_else.0;
                let new_vector: Vec<i32> = row
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != removed)
                    .map(|(_, &val)| val)
                    .collect();
                let result = rule_one(&new_vector);
                if result.is_empty() {
                    println!("row success==={:?}", new_vector);
                    sum += 1;
                    break;
                }
            }
            continue;
        }
    }

    println!("sum is {:?}", sum);
    // Split the line by tab and trim each field
    //let fields: Vec<u32> = line
    //    .split(' ')
    //    .map(|field| field.trim().parse::<u32>())
    //    .collect();
    //println!("{:?}", fields);
    Ok(())
}

//1 2 3 => min 2
//1 4 7 => max 6
//
//1 2 3 4 => min 3
//1 4 7 10 => max 9
//
//1 2 3 4 5 => min 4
//1 4 7 10 13 = max 12
//
//
//min = length - 1
//max = (length - 1) * 2
