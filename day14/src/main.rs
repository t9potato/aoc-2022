fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut x_max = 0;
    let mut x_min = usize::MAX;
    let mut y_min = usize::MAX;
    let mut y_max = 0;
    let mut coords = vec![];
    input.lines().for_each(|line|
         line.split(" -> ").map(|coord| {
            let x = coord.split(',').next().unwrap().parse::<usize>().unwrap();
            let y = coord.split(',').skip(1).next().unwrap().parse::<usize>().unwrap();
            if x > x_max {
                x_max = x
            } else if x < x_min {
                x_min = x
            }
            if y > y_max {
                y_max = y
            } else if y < y_min {
                y_min = y
            }
            (x, y)
        }).collect::<Vec<(usize, usize)>>().windows(2).for_each(|values| {
            if values[0].0 == values[1].0 {
                let mut lower = values[0].1;
                let mut higher = values[0].1;
                if values[1].1 > higher { higher = values[1].1 }
                else { lower = values[1].1 }
                for value in lower..=higher { 
                    coords.push((values[0].0, value))
                }
            } else {
                let mut lower = values[0].0;
                let mut higher = values[0].0;
                if values[1].0 > higher { higher = values[1].0 }
                else { lower = values[1].0 }
                for value in lower..=higher { 
                    coords.push((value, values[0].1));
                }
            }
        })
    );
    coords.sort();
    coords.dedup();
    let mut itterations = 0;
    'infinate: loop {
        let mut sand = (500, 0); 
        loop {
            if !coords.contains(&(sand.0, sand.1 + 1)) { sand = (sand.0, sand.1 + 1) }
            else if !coords.contains(&(sand.0 - 1, sand.1 + 1)) { sand = (sand.0 - 1, sand.1 + 1) }
            else if !coords.contains(&(sand.0 + 1, sand.1 + 1)) { sand = (sand.0 + 1, sand.1 + 1) }
            else {
                coords.push(sand);
                itterations += 1;
                break;
            }
            if sand.1 > y_max { break 'infinate; }
        }
    }
    itterations
}

fn part2(input: &str) -> usize {
    let mut x_max = 0;
    let mut x_min = usize::MAX;
    let mut y_min = usize::MAX;
    let mut y_max = 0;
    let mut coords = vec![];
    input.lines().for_each(|line|
         line.split(" -> ").map(|coord| {
            let x = coord.split(',').next().unwrap().parse::<usize>().unwrap();
            let y = coord.split(',').skip(1).next().unwrap().parse::<usize>().unwrap();
            if x > x_max {
                x_max = x
            } else if x < x_min {
                x_min = x
            }
            if y > y_max {
                y_max = y
            } else if y < y_min {
                y_min = y
            }
            (x, y)
        }).collect::<Vec<(usize, usize)>>().windows(2).for_each(|values| {
            if values[0].0 == values[1].0 {
                let mut lower = values[0].1;
                let mut higher = values[0].1;
                if values[1].1 > higher { higher = values[1].1 }
                else { lower = values[1].1 }
                for value in lower..=higher { 
                    coords.push((values[0].0, value))
                }
            } else {
                let mut lower = values[0].0;
                let mut higher = values[0].0;
                if values[1].0 > higher { higher = values[1].0 }
                else { lower = values[1].0 }
                for value in lower..=higher { 
                    coords.push((value, values[0].1));
                }
            }
        })
    );
    coords.sort();
    coords.dedup();
    let mut itterations = 0;
    'finished: loop {
        let mut sand = (500, 0); 
        loop {
            if !coords.contains(&(sand.0, sand.1 + 1)) { sand = (sand.0, sand.1 + 1) }
            else if !coords.contains(&(sand.0 - 1, sand.1 + 1)) { sand = (sand.0 - 1, sand.1 + 1) }
            else if !coords.contains(&(sand.0 + 1, sand.1 + 1)) { sand = (sand.0 + 1, sand.1 + 1) }
            else {
                coords.push(sand);
                itterations += 1;
                if sand == (500, 0) { break 'finished; }
                break;
            }
            if sand.1 == y_max + 1 { 
                coords.push(sand);
                itterations += 1;
                if sand == (500, 0) { break 'finished; }
                break;
            }
        }
    }
    itterations
}
