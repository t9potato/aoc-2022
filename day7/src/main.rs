use std::collections::HashMap;

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut path = vec![""];
    let mut directories: HashMap<String, usize> = HashMap::new();
    input.lines().for_each(|item| {
        let item = item.split(" ").collect::<Vec<&str>>();
        if item[1] == "cd" && item[2] == ".." {
            let value = directories[&path.join("/")];
            path.pop();
            *directories.get_mut(&path.join("/")).unwrap() += value;
        } else if item[1] == "cd" {
            path.push(item[2]);
            let itm = directories.insert(path.join("/"), 0);
            if let Some(itm) = itm {
                let mut paths = path.clone();
                paths.pop();
                *directories.get_mut(&paths.join("/")).unwrap() -= itm;
            }
        } else if item[0] != "dir" && item[0] != "$" {
            *directories.get_mut(&path.join("/")).unwrap() += item[0].parse::<usize>().unwrap();
        }
    });
    directories.values().filter(|val| {
        val <= &&100000
    }).sum()
}

fn part2(input: &str) -> usize {
}
