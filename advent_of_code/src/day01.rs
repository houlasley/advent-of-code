
pub fn part_one() -> Option<u64> {
    let data = read_file_vec("day01.txt");
    data
}


fn read_file_vec(filepath: &str) -> Option<u64> {
    let input = std::fs::read_to_string(filepath).unwrap();
    let mut max_sum = 0;
    for group in input.split("\n\n") {
        let mut sum: u64 = 0;
        for line in group.lines() {
            let value = line.parse::<u64>().unwrap();
            sum += value;
        }
        if sum > max_sum {
            max_sum = sum
        }
    }
    println!("Max sum is {}", max_sum);


    Some(max_sum)
}

