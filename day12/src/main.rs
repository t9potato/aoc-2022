use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let input = include_str!("input");
    println!("part 1: {}\npart 2: {}", part1(input), part2(input));
}


fn part1(input: &str) -> usize {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut grid: Vec<Vec<usize>> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        grid.push(line.chars().enumerate().map(|(x, letter)| {
            if letter == 'S' { start = (x, y); 0 }
            else if letter == 'E' { end = (x, y); 25 }
            else { letter.to_string().as_bytes()[0] as usize - 97 }
        }).collect());
    });
    let mut options: Vec<Vec<usize>> = vec![];
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            options.push(vec![]);
            if y != 0 && value + 1 >= grid[y - 1][x] {
                options[(y*grid[0].len())+x].push(x + (grid[0].len() * (y-1)));
            }
            if y != grid.len() - 1 && value + 1 >= grid[y + 1][x] {
                options[(y*grid[0].len())+x].push(x + (grid[0].len() * (y+1)));
            }
            if x != 0 && value + 1 >= grid[y][x - 1] {
                options[(y*grid[0].len())+x].push(x-1 + (grid[0].len() * y));
            }
            if x != grid[0].len() -1 && value + 1 >= grid[y][x + 1] {
                options[(y*grid[0].len())+x].push(1+x + (grid[0].len() * y));
            }
        })
    });

    dijkstra(&options, (start.1 * grid[0].len()) + start.0, (end.1 * grid[0].len()) + end.0).unwrap()
}

fn dijkstra(adj_list: &Vec<Vec<usize>>, start: usize, end: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end { return Some(cost); }
        if cost > dist[position] { continue; }
        for edge in &adj_list[position] {
            let next = State { cost: cost + 1, position: *edge };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn part2(input: &str) -> usize {
    let mut start = (0, 0);
    let mut grid: Vec<Vec<usize>> = vec![];
    input.lines().enumerate().for_each(|(y, line)| {
        grid.push(line.chars().enumerate().map(|(x, letter)| {
            if letter == 'S' { 1 }
            else if letter == 'E' { start = (x, y); 26 }
            else { letter.to_string().as_bytes()[0] as usize - 96 }
        }).collect());
    });
    let mut options: Vec<(Vec<usize>, usize)> = vec![];
    grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, value)| {
            options.push((vec![], *value));
            if y != 0 && value - 1 <= grid[y - 1][x] {
                options[(y*grid[0].len())+x].0.push(x + (grid[0].len() * (y-1)));
            }
            if y != grid.len() - 1 && value - 1 <= grid[y + 1][x] {
                options[(y*grid[0].len())+x].0.push(x + (grid[0].len() * (y+1)));
            }
            if x != 0 && value - 1 <= grid[y][x - 1] {
                options[(y*grid[0].len())+x].0.push(x-1 + (grid[0].len() * y));
            }
            if x != grid[0].len() -1 && value - 1 <= grid[y][x + 1] {
                options[(y*grid[0].len())+x].0.push(1+x + (grid[0].len() * y));
            }
        })
    });

    println!("{:?}", options);
    dijkstra2(&options, (start.1 * grid[0].len()) + start.0).unwrap()
}

fn dijkstra2(adj_list: &Vec<(Vec<usize>, usize)>, start: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if  adj_list[position].1 == 1 { return Some(cost); }
        if cost > dist[position] { continue; }
        for edge in &adj_list[position].0 {
            let next = State { cost: cost + 1, position: *edge };
            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}
