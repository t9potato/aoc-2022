fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut score = 0;
    input
        .lines()
        .map(|game| {
            game.split(' ')
                .enumerate()
                .map(|(i, turn)| turn.as_bytes()[0] as usize - 65 - (i as usize * 23))
                .collect::<Vec<usize>>()
        })
        .for_each(|game| {
            score += game[1] + 1;
            if game[0] == game[1] {
                score += 3;
            } else if game[0] + 1 == game[1] || game[0] == 2 && game[1] == 0 {
                score += 6;
            }
        });
    score
}

fn part2(input: &str) -> usize {
    let mut score = 0;
    input
        .lines()
        .map(|game| {
            game.split(' ')
                .enumerate()
                .map(|(i, turn)| turn.as_bytes()[0] as usize - 65 - (i as usize * 23))
                .collect::<Vec<usize>>()
        })
        .for_each(|game| {
            score += game[1] * 3;
            score += ((game[1] + 2) % 3 + game[0]) % 3 + 1;
        });
    score
}
