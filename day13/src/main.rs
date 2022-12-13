use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

fn main() {
    let input = include_str!("input");
    println!("Part 1: {}\nPart 2: {}", part1(input), part2(input));
}

#[derive(Debug, Clone)]
enum Packet {
    Int(usize),
    List(Vec<Packet>),
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(std::cmp::Ordering::Equal)
    }
}
impl std::cmp::Eq for Packet {}
impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // println!("Comparing {:?} and {:?}", self, other);
        match (self, other ) {
            (Packet::Int(l), Packet::Int(r)) => { l.partial_cmp(r) },
            (Packet::List(l), Packet::List(r)) => {
                for (l, r) in l.iter().zip(r) {
                    if let Some(result) = l.partial_cmp(r) {
                        if result != std::cmp::Ordering::Equal {
                            return Some(result);
                        }
                    }
                }
                l.len().partial_cmp(&r.len())
            },
            (Packet::List(_), Packet::Int(_)) => self.partial_cmp(&Packet::List(vec![other.clone()])),
            (Packet::Int(_), Packet::List(_)) => Packet::List(vec![self.clone()]).partial_cmp(&other),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut ans = 0;
    input.split("\n\n").enumerate().for_each(|(i, package)| {
        let left = parse(package.lines().next().unwrap()).unwrap().1;
        let right = parse(package.lines().skip(1).next().unwrap()).unwrap().1;
        if left < right { ans += 1+i }
    });
    ans
}

fn parse(input: &str) -> IResult<&str, Packet>{
    alt((
            map(u32, |v| Packet::Int(v as usize)),
            map(
                delimited(tag("["), separated_list0(tag(","), parse), tag("]")),
                |v| Packet::List(v)
               )
        ))(input)
}

fn part2(input: &str) -> usize {
    let mut packages: Vec<Packet> = vec![];
    input.split("\n\n").for_each(|package| {
        vec![
            parse(package.lines().next().unwrap()).unwrap().1,
            parse(package.lines().skip(1).next().unwrap()).unwrap().1
        ].iter().for_each(|packet| packages.push(packet.clone()))
    });
    packages.push(Packet::List(vec![Packet::List(vec![Packet::Int(2)])]));
    packages.push(Packet::List(vec![Packet::List(vec![Packet::Int(6)])]));
    packages.sort();
    let mut sep1 = 0;
    let mut sep2 = 0;
    packages.iter().enumerate().for_each(|(i, package)| {
        if package == &Packet::List(vec![Packet::List(vec![Packet::Int(2)])]) {
            sep1 = i + 1;
        } else if package == &Packet::List(vec![Packet::List(vec![Packet::Int(6)])]) {
            sep2 = i + 1;
        }
    });
    sep1 * sep2
}
