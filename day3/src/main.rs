use std::char;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut total = 0;
    input.lines().map(|line| {
        let len = line.len();
        let sides = line.split_at(len / 2);
        let update = |side: &str| side.chars().map(|char| {
                let case = char.is_uppercase();
                char.to_lowercase().to_string().bytes().next().unwrap() as usize - 96 + match case {
                    true => 26,
                    false => 0
                }
            }).collect::<Vec<usize>>();
        let mut content = (update(sides.0), update(sides.1));
        content.0.sort();
        content.1.sort();
        content
    }).for_each(|pack| {
        'top: for item in pack.0 {
            for element in pack.1.iter() {
                if element > &item { break; }
                else if element == &item {
                    total += element;
                    break 'top;
                }
            }
        }
    });
    total
}

fn part2(input: &str) -> usize {
    let mut total = 0;
    let packs = input.lines().map(|line| {
        let mut update = line.chars().map(|char| {
                let case = char.is_uppercase();
                char.to_lowercase().to_string().bytes().next().unwrap() as usize - 96 + match case {
                    true => 26,
                    false => 0
                }
            }).collect::<Vec<usize>>();
        update.sort();
        update
    }).collect::<Vec<Vec<usize>>>();
    for i in 0..(packs.len() / 3) {
        for char in packs[i * 3].iter() {
            if packs[i*3 + 1].contains(&char) && packs[i*3 +2].contains(&char) {
                total += char;
                break;
            }
        }
    }
    total
}
