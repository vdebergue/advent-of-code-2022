use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let out1 = run1(contents.clone());
    println!("Part 1 => {}", String::from_iter(out1));

    let out2 = run2(contents.clone());
    println!("Part 2 => {}", String::from_iter(out2));
}

fn run1(contents: String) -> Vec<char> {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let stacks_str = parts[0];
    let moves_str = parts[1];

    let mut stacks = read_stacks(stacks_str);
    moves_str
        .lines()
        .filter(|l| !l.is_empty())
        .for_each(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let count = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap();
            let to = parts[5].parse::<usize>().unwrap();
            for _ in 0..count {
                let elem = stacks[from - 1].pop_front().unwrap();
                stacks[to - 1].push_front(elem);
            }

            // for i in 0..size {
            //     println!("Stack {i}, {:?}", stacks[i]);
            // }
        });
    let out: Vec<char> = stacks.iter().map(|s| s[0]).collect();
    out
}

fn run2(contents: String) -> Vec<char> {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let stacks_str = parts[0];
    let moves_str = parts[1];

    let mut stacks = read_stacks(stacks_str);
    moves_str
        .lines()
        .filter(|l| !l.is_empty())
        .for_each(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            let count = parts[1].parse::<usize>().unwrap();
            let from = parts[3].parse::<usize>().unwrap();
            let to = parts[5].parse::<usize>().unwrap();
            let mut tmp = Vec::with_capacity(count);
            for _ in 0..count {
                let elem = stacks[from - 1].pop_front().unwrap();
                tmp.push(elem);
            }
            for i in (0..count).rev() {
                stacks[to - 1].push_front(tmp[i])
            }

            // for i in 0..size {
            //     println!("Stack {i}, {:?}", stacks[i]);
            // }
        });
    let out: Vec<char> = stacks.iter().map(|s| s[0]).collect();
    out
}

fn read_stacks(stacks_str: &str) -> Vec<VecDeque<char>> {
    let size = stacks_str
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .count();
    let height = stacks_str.lines().count() - 1;
    // Top of stack is at front
    let mut stacks: Vec<VecDeque<char>> = vec![VecDeque::new(); size];
    stacks_str.lines().take(height).for_each(|line| {
        for idx in 0..size {
            let pos = (idx) * 4 + 1;
            let c = line.chars().nth(pos).unwrap();
            if c != ' ' {
                stacks[idx].push_back(c);
            }
        }
    });
    for i in 0..size {
        println!("Stack {i}, {:?}", stacks[i]);
    }
    stacks
}
