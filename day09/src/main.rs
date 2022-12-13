use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut positions: Vec<(isize, isize)> = vec![(0,0)];
    input.lines().for_each(|line| {
        let direction = line.chars().next().unwrap();
        let value = line.split(' ').skip(1).next().unwrap().to_string().parse::<usize>().unwrap();
        for _ in 0..value {
            match direction {
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => panic!("Invalid command")
            }
            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail.0 += (head.0 - tail.0).clamp(-1, 1);
                tail.1 += (head.1 - tail.1).clamp(-1, 1);
                positions.push(tail);
            }
        }
    });
    positions.iter().unique().collect::<Vec<&(isize, isize)>>().len()
}

fn part2(input: &str) -> usize {
    let mut head: (isize, isize) = (0, 0);
    let mut h1: (isize, isize) = (0, 0);
    let mut h2: (isize, isize) = (0, 0);
    let mut h3: (isize, isize) = (0, 0);
    let mut h4: (isize, isize) = (0, 0);
    let mut h5: (isize, isize) = (0, 0);
    let mut h6: (isize, isize) = (0, 0);
    let mut h7: (isize, isize) = (0, 0);
    let mut h8: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut positions: Vec<(isize, isize)> = vec![(0,0)];
    input.lines().for_each(|line| {
        let direction = line.chars().next().unwrap();
        let value = line.split(' ').skip(1).next().unwrap().to_string().parse::<usize>().unwrap();
        for _ in 0..value {
            match direction {
                'R' => head.0 += 1,
                'L' => head.0 -= 1,
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                _ => panic!("Invalid command")
            }
            if (head.0 - h1.0).abs() > 1 || (head.1 - h1.1).abs() > 1 {
                h1.0 += (head.0 - h1.0).clamp(-1, 1);
                h1.1 += (head.1 - h1.1).clamp(-1, 1);
            }
            if (h1.0 - h2.0).abs() > 1 || (h1.1 - h2.1).abs() > 1 {
                h2.0 += (h1.0 - h2.0).clamp(-1, 1);
                h2.1 += (h1.1 - h2.1).clamp(-1, 1);
            }
            if (h2.0 - h3.0).abs() > 1 || (h2.1 - h3.1).abs() > 1 {
                h3.0 += (h2.0 - h3.0).clamp(-1, 1);
                h3.1 += (h2.1 - h3.1).clamp(-1, 1);
            }
            if (h3.0 - h4.0).abs() > 1 || (h3.1 - h4.1).abs() > 1 {
                h4.0 += (h3.0 - h4.0).clamp(-1, 1);
                h4.1 += (h3.1 - h4.1).clamp(-1, 1);
            }
            if (h4.0 - h5.0).abs() > 1 || (h4.1 - h5.1).abs() > 1 {
                h5.0 += (h4.0 - h5.0).clamp(-1, 1);
                h5.1 += (h4.1 - h5.1).clamp(-1, 1);
            }
            if (h5.0 - h6.0).abs() > 1 || (h5.1 - h6.1).abs() > 1 {
                h6.0 += (h5.0 - h6.0).clamp(-1, 1);
                h6.1 += (h5.1 - h6.1).clamp(-1, 1);
            }
            if (h6.0 - h7.0).abs() > 1 || (h6.1 - h7.1).abs() > 1 {
                h7.0 += (h6.0 - h7.0).clamp(-1, 1);
                h7.1 += (h6.1 - h7.1).clamp(-1, 1);
            }
            if (h7.0 - h8.0).abs() > 1 || (h7.1 - h8.1).abs() > 1 {
                h8.0 += (h7.0 - h8.0).clamp(-1, 1);
                h8.1 += (h7.1 - h8.1).clamp(-1, 1);
            }
            if (h8.0 - tail.0).abs() > 1 || (h8.1 - tail.1).abs() > 1 {
                tail.0 += (h8.0 - tail.0).clamp(-1, 1);
                tail.1 += (h8.1 - tail.1).clamp(-1, 1);
                positions.push(tail);
            }
        }
    });
    positions.iter().unique().collect::<Vec<&(isize, isize)>>().len()
}
