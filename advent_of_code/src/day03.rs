pub fn part_one() -> u32 {
    let res = file_in("day_03.txt");
    return res;
}

pub fn part_two() -> usize {
    let res = file_in_two("day_03.txt");
    return res
}

fn file_in_two(filepath: &str) -> usize {
    let mut val: usize = 0;
    let input = std::fs::read_to_string(filepath).unwrap();

    let input_iter: Vec<&str> = input
        .split("\n")
        .collect();

    let mut iter = input_iter.iter();
    while let Some(first) = iter.next() {
        let Some(second) = iter.next() else { break; };
        let Some(third) = iter.next() else { break; };
        val += find_common_character(first, &second, third);
    }


    return val;
}

fn find_common_character(chunk1: &str, chunk2: &str, chunk3: &str) -> usize {
    let mut val: usize = 0;
    for i in chunk1.chars() {
        if chunk2.contains(i) && chunk3.contains(i) {
            val = calculate_value(i);
            break;
        }
    }
    return val;
}

fn file_in(filepath: &str) -> u32 {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut priority_vals = Vec::new();
    for group in input.split("\n\n") {
        for line in group.lines() {
            let splits = line.split_at(line.len() / 2);
            let char = find_character(splits);
            let val = calculate_value(char) as u32;
            priority_vals.push(val);
        }
    }
    let total_sum = priority_vals.iter().sum();
    return total_sum;
}


fn find_character(line: (&str, &str)) -> char {
    let var = '\0';
    for char in line.0.chars() {
        if line.1.contains(char) {
            return char;
        }
    }
    return var;
}

fn calculate_value(character: char) -> usize {
    let mut val: usize = 0;
    let mut alphabet = ('a'..='z').into_iter().collect::<Vec<char>>();
    alphabet.append(&mut ('A'..='Z').into_iter().collect::<Vec<char>>());

    for i in 0..alphabet.len() {
        let x = alphabet[i];
        if x == character {
            val = i + 1;
            break;
        } else {
            val = 0;
        }
    }

    return val;
}
