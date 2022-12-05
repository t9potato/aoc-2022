use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> String {
    let mut crane: HashMap<usize, Vec<char>> = HashMap::new();
    let mut input = input.split("\n\n");
    let _ = input.next().unwrap().lines().for_each(|line| {
        for symbol in line
            .chars()
            .enumerate()
            .filter(|char| char.1.is_alphabetic())
            .collect::<Vec<(usize, char)>>()
        {
            let mut input = crane
                .get(&((symbol.0 - 1) / 4))
                .unwrap_or(&Vec::new())
                .clone();
            input.push(symbol.1);
            crane.insert((symbol.0 - 1) / 4, input);
        }
    });
    crane.clone().iter().for_each(|item| {
        let mut out = item.1.clone();
        out.reverse();
        crane.insert(*item.0, out);
    });
    input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .enumerate()
                .filter(|symbol| symbol.0 % 2 == 1)
                .map(|symbol| symbol.1.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .for_each(|item| {
            let mut input = crane[&(item[1] - 1)].clone();
            let mut output = crane[&(item[2] - 1)].clone();
            for _ in 0..item[0] {
                output.push(input.pop().unwrap())
            }
            crane.insert(item[1] - 1, input);
            crane.insert(item[2] - 1, output);
        });
    let mut output = String::new();
    for i in 0..crane.len() {
        output = format!("{}{}", output, crane[&i][crane[&i].len() - 1]);
    }
    output
}

fn part2(input: &str) -> String {
    let mut crane: HashMap<usize, Vec<char>> = HashMap::new();
    let mut input = input.split("\n\n");
    let _ = input.next().unwrap().lines().for_each(|line| {
        for symbol in line
            .chars()
            .enumerate()
            .filter(|char| char.1.is_alphabetic())
            .collect::<Vec<(usize, char)>>()
        {
            let mut input = crane
                .get(&((symbol.0 - 1) / 4))
                .unwrap_or(&Vec::new())
                .clone();
            input.push(symbol.1);
            crane.insert((symbol.0 - 1) / 4, input);
        }
    });
    crane.clone().iter().for_each(|item| {
        let mut out = item.1.clone();
        out.reverse();
        crane.insert(*item.0, out);
    });
    input
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(' ')
                .enumerate()
                .filter(|symbol| symbol.0 % 2 == 1)
                .map(|symbol| symbol.1.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .for_each(|item| {
            let input = crane[&(item[1] - 1)].clone();
            let input = input.split_at(crane[&(item[1] - 1)].len() - item[0]);
            let mut output = crane[&(item[2] - 1)].clone();
            crane.insert(item[1] - 1, input.0.to_vec());
            output.append(&mut input.1.to_vec());
            crane.insert(item[2] - 1, output);
        });
    let mut output = String::new();
    for i in 0..crane.len() {
        output = format!("{}{}", output, crane[&i][crane[&i].len() - 1]);
    }
    output
}
