fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|side| {
                    side.split('-')
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>()
                })
                .collect::<Vec<Vec<isize>>>()
        })
        .filter(|item| {
            (item[0][0] <= item[1][0] && item[0][1] >= item[1][1])
                || (item[0][0] >= item[1][0] && item[0][1] <= item[1][1])
        })
        .collect::<Vec<Vec<Vec<isize>>>>()
        .len()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|side| {
                    side.split('-')
                        .map(|num| num.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>()
                })
                .collect::<Vec<Vec<isize>>>()
        })
        .filter(|item| {
            (item[0][0] <= item[1][1] && item[0][1] >= item[1][0])
                || (item[0][0] >= item[1][1] && item[0][1] <= item[1][0])
        })
        .collect::<Vec<Vec<Vec<isize>>>>()
        .len()
}
