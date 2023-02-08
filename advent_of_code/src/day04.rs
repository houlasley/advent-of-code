pub fn part_one() -> u32 {
    let file = file_in();

    let mut val: u32 = 0;

    let file_iter: Vec<&str> = file
        .split("\n")
        .collect();


    for i in file_iter {
        let j: Vec<&str> = i
            .split(',')
            .collect();
        if compare_pairs(j) == true {
            val += 1;
        }
    }
    return val;
}

fn compare_pairs(pairs: Vec<&str>) -> bool {
    let pair1: Vec<u16> = pairs[0]
        .split("-")
        .map(|x| x.parse::<u16>()
            .unwrap())
        .collect();
    let pair2: Vec<u16> = pairs[1]
        .split("-")
        .map(|x| x.parse::<u16>()
            .unwrap())
        .collect();

    let pair1_range = &pair1[0]..&(&pair1[1] + 1);
    let pair2_range = &pair2[0]..&(&pair2[1] + 1);

    return if pair1_range.contains(&&pair2[0]) && pair1_range.contains(&&pair2[1]) {
        true
    } else if pair2_range.contains(&&pair1[0]) && pair2_range.contains(&&pair1[1]) {
        true
    } else { false }

}

pub fn part_two() -> u32 {
    return 0;
}

fn file_in() -> String {
    return std::fs::read_to_string("day04.txt").unwrap();
}