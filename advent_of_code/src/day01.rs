
pub fn part_one() -> u64 {
    let data = file_to_vec("day01.txt");
    return data[0]
}

pub fn part_two() -> u64 {
    let data = file_to_vec("day01.txt");

    return data[0..5].iter().sum()
}

fn file_to_vec(filepath: &str) -> Vec<u64> {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut max_sum = 0;
    let mut output_vec = Vec::new();
    for group in input.split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();

            output_vec.push(value);
            sum += value;
        }
        if sum > max_sum {
            max_sum = sum
        }
    }
    output_vec.sort();
    output_vec.reverse();
    return output_vec
}



