use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("sample.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut sum: u32 = 0;
    println!("{}", contents);
    let char_digits = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];
    for (search, replacement) in char_digits {
        contents = contents.replace(search, replacement);
    }
    println!("{}", contents);

    for line in contents.lines() {
        let char_digits = vec![
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ];
        for (search, replacement) in char_digits {
            if SSline.find(search).is_some() {
                println!("find==={:?}{:?}", search, line.find(search));
            }
        }
        let digits: Vec<char> = line
            .chars()
            .filter_map(|x| if x.is_digit(10) { Some(x) } else { None })
            .collect();
        println!("digits==={:?}", digits);
        let first = digits.first().unwrap().to_string();
        let last = digits.last().unwrap();
        let combined = format!("{}{}", first, last);
        println!("Combined=={}", combined);
        sum += combined.parse::<u32>().unwrap();
    }
    println!("sum==={:?}", sum);

    Ok(())
}
