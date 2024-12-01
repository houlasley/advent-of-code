use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input.txt";

    // Open the file
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut column1: Vec<i64> = Vec::new();
    let mut column2: Vec<i64> = Vec::new();
    // Read each line
    for line in reader.lines() {
        let line = line?;

        // Split the line by tab and trim each field
        let fields: Vec<&str> = line.split("   ").map(|field| field.trim()).collect();

        match (fields[0].parse::<i64>(), fields[1].parse::<i64>()) {
            (Ok(num1), Ok(num2)) => {
                column1.push(num1);
                column2.push(num2);
            }
            _ => {
                eprintln!("Skippings invalid line: {}", line);
            }
        }
    }
    fn part_one(column1: &mut Vec<i64>, column2: &mut Vec<i64>) -> i64 {
        column1.sort();
        column2.sort();
        let differences: i64 = column1
            .iter()
            .zip(column2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        println!("{:?}", differences);
        differences
    }
    part_one(&mut column1, &mut column2);

    fn part_two(column1: &mut Vec<i64>, column2: &mut Vec<i64>) -> i64 {
        let mut map: HashMap<i64, i64> = HashMap::new();
        for value in column2 {
            *map.entry(*value).or_insert(0) += 1;
        }
        let similarity: i64 = column1.iter().map(|a| (a * map.get(a).unwrap_or(&0))).sum();
        println!("{:?}", similarity);
        similarity
    }
    part_two(&mut column1, &mut column2);
    Ok(())
}
