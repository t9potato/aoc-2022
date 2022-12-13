fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut chars = vec![];
    for char in input.chars().enumerate() {
        if chars.len() == 4 {
            chars.remove(0);
        }
        chars.push(char.1);
        let mut test = chars.clone();
        test.sort();
        test.dedup();
        if test.len() == 4 {
            return char.0 + 1;
        }
    }
    0
}

fn part2(input: &str) -> usize {
    let mut chars = vec![];
    for char in input.chars().enumerate() {
        if chars.len() == 14 {
            chars.remove(0);
        }
        chars.push(char.1);
        let mut test = chars.clone();
        test.sort();
        test.dedup();
        if test.len() == 14 {
            return char.0 + 1;
        }
    }
    0
}
