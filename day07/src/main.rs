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
    let mut path = vec![""];
    let mut directories: HashMap<String, usize> = HashMap::new();
    let mut root_val = 0;
    input.lines().for_each(|line|{
        if line.starts_with("$ cd ..") {
            let path_val = *directories.get(&path.join("/")).unwrap();
            path.pop();
            *directories.get_mut(&path.join("/")).unwrap() += path_val;
        } else if line.starts_with("$ cd") {
            let new_path = line.split(" ").skip(2).next().unwrap();
            let item = *directories.get(&format!("{}/{}", path.join("/"), new_path).to_string()).unwrap_or(&0);
            *directories.get_mut(&path.join("/")).unwrap_or(&mut 0) -= item;
            path.push(new_path);
            directories.insert(path.join("/"), item);
        } else if !line.starts_with("$") && !line.starts_with("dir") {
            *directories.get_mut(&path.join("/")).unwrap() += line.split(" ").next().unwrap().parse::<usize>().unwrap();
            root_val += line.split(" ").next().unwrap().parse::<usize>().unwrap();
        }
    });
    directories.remove(&"//".to_string());
    directories.insert("//".to_string(), root_val);
    let val = directories.values().max().unwrap();
    let mut directories = directories.values().map(|value| *value).collect::<Vec<usize>>();
    directories.sort();
    *directories.iter().filter(|value| **value >= val - 40000000).next().unwrap()
}
