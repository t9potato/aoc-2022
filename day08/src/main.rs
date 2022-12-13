fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let x_max = input.lines().next().unwrap().chars().collect::<Vec<char>>().len() - 1;
    let y_max = input.lines().collect::<Vec<&str>>().len() - 1;
    let forest = input
        .lines()
        .map(|line| line
             .chars()
             .map(|char| char.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()
         ).collect::<Vec<Vec<usize>>>();
    let mut ans = 0;
    for (y, collum) in forest.iter().enumerate() {
        for (x, element) in collum.iter().enumerate() {
            let mut found = 0;
            if x == 0 || x == x_max || y ==0 || y == y_max {
                ans += 1;
                continue;
            }
            for index in (x..=x_max).skip(1) {
                if forest[y][index] >= *element {
                    found += 1;
                    break;
                }
            }
            for index in 0..x {
                if forest[y][index] >= *element {
                    found += 1;
                    break;
                }
            }
            for index in (y..=y_max).skip(1) {
                if forest[index][x] >= *element {
                    found += 1;
                    break;
                }
            }
            for index in 0..y {
                if forest[index][x] >= *element {
                    found += 1;
                    break;
                }
            }
            if found != 4{
                ans += 1;
            }
        }
    }
    ans
}

fn part2(input: &str) -> usize {
    let x_max = input.lines().next().unwrap().chars().collect::<Vec<char>>().len() - 1;
    let y_max = input.lines().collect::<Vec<&str>>().len() - 1;
    let forest = input
        .lines()
        .map(|line| line
             .chars()
             .map(|char| char.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>()
         ).collect::<Vec<Vec<usize>>>();
    let mut ans = 0;
    for (y, collum) in forest.iter().enumerate() {
        for (x, element) in collum.iter().enumerate() {
            let mut x_up = 0;
            let mut x_down = 0;
            let mut y_up = 0;
            let mut y_down = 0;
            for index in (x..=x_max).skip(1) {
                x_up += 1;
                if forest[y][index] >= *element {
                    break;
                }
            }
            for index in (0..x).rev() {
                x_down += 1;
                if forest[y][index] >= *element {
                    break;
                }
            }
            for index in (y..=y_max).skip(1) {
                y_up += 1;
                if forest[index][x] >= *element {
                    break;
                }
            }
            for index in (0..y).rev() {
                y_down += 1;
                if forest[index][x] >= *element {
                    break;
                }
            }
            let tmp = x_up * x_down * y_up * y_down;
            println!("{}", tmp);
            if tmp > ans {
                ans = tmp
            }
        }
    }
    ans
}
