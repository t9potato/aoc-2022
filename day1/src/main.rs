fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut max = 0;
    input.split("\n\n").for_each(|package| {
        let mut sum = 0;
        package.lines().for_each(|num| {
            sum += num.parse::<usize>().unwrap();
        });
        if sum > max {
            max = sum;
        }
    });
    max
}

fn part2(input: &str) -> usize {
    let mut max: Vec<usize> = vec![0, 0, 0];
    input.split("\n\n").for_each(|package| {
        let mut sum = 0;
        package.lines().for_each(|num| {
            sum += num.parse::<usize>().unwrap();
        });
        max.push(sum);
        max.sort();
        max.reverse();
        max.pop();
    });
    max.iter().sum()
}
