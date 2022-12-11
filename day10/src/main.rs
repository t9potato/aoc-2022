fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut x = 1;
    let mut cycle = 0;
    input.lines().map(|line| {
        let mut ret = 0;
        cycle += 1;
        if [20, 60, 100, 140, 180, 220].contains(&cycle) {
            ret = cycle * x;
        }
        if line.split(' ').next().unwrap() == "addx" {
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                ret = cycle * x;
            }
            x += line.split(' ').skip(1).next().unwrap().parse::<isize>().unwrap();
        }
        ret
    }).sum::<isize>() as usize
}

fn part2(input: &str) -> String {
    const SCREEN_LENGHT: usize = 40;
    let mut x = 1;
    let mut scan_pos = 0;
    let mut line_num = 0;
    let mut screen: Vec<Vec<char>> = vec![vec![]];
    input.lines().for_each(|line| {
        screen[line_num].push('.');
        if scan_pos >= x.clamp(1, 555) - 1 && scan_pos <= x + 1 {
            screen[line_num][scan_pos] = '█';
        }
        scan_pos += 1;
        if scan_pos == SCREEN_LENGHT { scan_pos = 0; line_num += 1; screen.push(vec![]) }
        if line.split(' ').next().unwrap() == "addx" {
            screen[line_num].push('.');
            if scan_pos >= x.clamp(1, 666) - 1 && scan_pos <= x+1 {
                screen[line_num][scan_pos] = '█';
            }
            scan_pos += 1;
            if scan_pos == SCREEN_LENGHT { scan_pos = 0; line_num += 1; screen.push(vec![]) }
            x = (x as isize + line.split(' ').skip(1).next().unwrap().parse::<isize>().unwrap()) as usize;
        }
    });
    ("\n".to_string() + screen.iter().map(|line|{
        line.iter().collect::<String>() + "\n"
    }).collect::<String>().as_str()).trim_end().to_string()
}
