use regex::Regex;

struct Monkey {
    items: Vec<usize>,
    interactions: usize,
    opperation_string: String,
    opperation: Box<dyn Fn(usize, String) -> usize>,
    test: Box<dyn Fn(usize, usize, usize, usize) -> usize>,
    divisible: usize,
    true_outcome: usize,
    false_outcome: usize,
}

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = vec![];
    input.split("\n\n").for_each(|monkey| {
        let mut lines = monkey.lines().skip(1);
        let items = lines.next().unwrap().trim().split(" ").skip(2).map(|item| item.trim_end_matches(',').parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let opperation_string = lines.next().unwrap().trim().split(" ").skip(4).collect::<String>();
        let opperation = Box::from(|num: usize, string: String| {
            if string.chars().next().unwrap() == '+' {
                num + string.chars().skip(1).collect::<String>().parse::<usize>().unwrap()
            } else {
                let out = string.chars().skip(1).collect::<String>();
                if out == "old" {
                    num * num
                } else {
                    num * out.parse::<usize>().unwrap()
                }
            }
        });

        let divisible = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let true_outcome = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let false_outcome = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let test = Box::from(|input, divisible, true_outcome, false_outcome| if input % divisible == 0 { true_outcome } else { false_outcome });
        monkeys.push(Monkey { items, interactions: 0, opperation_string, opperation, test, divisible, true_outcome, false_outcome });
    });
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                let item = (monkeys[i].opperation)(item, monkeys[i].opperation_string.clone()) / 3;
                let index = (monkeys[i].test)(item, monkeys[i].divisible, monkeys[i].true_outcome, monkeys[i].false_outcome);
                monkeys[index].items.push(item);
                monkeys[i].interactions += 1;
            }
            monkeys[i].items = vec![]
        }
    }
    let mut first = 0;
    let mut second = 0;
    for monkey in monkeys {
        if monkey.interactions >= second {
            if monkey.interactions >= first {
                second = first;
                first = monkey.interactions;
            } else {
                second = monkey.interactions
            }
        }
    }
    first * second
}

fn part2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut devisors: Vec<usize> = vec![];
    input.split("\n\n").for_each(|monkey| {
        let mut lines = monkey.lines().skip(1);
        let items = lines.next().unwrap().trim().split(" ").skip(2).map(|item| item.trim_end_matches(',').parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let opperation_string = lines.next().unwrap().trim().split(" ").skip(4).collect::<String>();
        let opperation = Box::from(|num: usize, string: String| {
            if string.chars().next().unwrap() == '+' {
                num + string.chars().skip(1).collect::<String>().parse::<usize>().unwrap()
            } else {
                let out = string.chars().skip(1).collect::<String>();
                if out == "old" {
                    num * num
                } else {
                    num * out.parse::<usize>().unwrap()
                }
            }
        });

        let divisible = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        devisors.push(divisible);
        let true_outcome = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let false_outcome = lines.next().unwrap().trim().split(" ").last().unwrap().parse::<usize>().unwrap();
        let test = Box::from(|input, divisible, true_outcome, false_outcome| if input % divisible == 0 { true_outcome } else { false_outcome });
        monkeys.push(Monkey { items, interactions: 0, opperation_string, opperation, test, divisible, true_outcome, false_outcome });
    });
    let lcm: usize = devisors.iter().product();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for item in monkeys[i].items.clone() {
                let item = (monkeys[i].opperation)(item, monkeys[i].opperation_string.clone()) % lcm;
                let index = (monkeys[i].test)(item, monkeys[i].divisible, monkeys[i].true_outcome, monkeys[i].false_outcome);
                monkeys[index].items.push(item);
                monkeys[i].interactions += 1;
            }
            monkeys[i].items = vec![]
        }
    }
    let mut first = 0;
    let mut second = 0;
    for monkey in monkeys {
        if monkey.interactions >= second {
            if monkey.interactions >= first {
                second = first;
                first = monkey.interactions;
            } else {
                second = monkey.interactions
            }
        }
    }
    first * second
}
