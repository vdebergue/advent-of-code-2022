use std::collections::VecDeque;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let total = run1(contents.clone());
    println!("Total is {total}");

    let total2 = run2(contents);
    println!("Total part 2 is {total2}");
}

fn run1(contents: String) -> u64 {
    let mut state = State::read(&contents, true);
    println!("Init State: {state:?}");
    for _ in 0..20 {
        state.round();
    }
    state.monkey_business()
}

fn run2(contents: String) -> u64 {
    let mut state = State::read(&contents, false);
    println!("Init State: {state:?}");
    for i in 0..10000 {
        // println!("Round {i}...");
        state.round();
    }
    state.monkey_business()
}

type Item = u64;

struct Monkey {
    items: VecDeque<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test: Box<dyn Fn(Item) -> usize>,
    inspections: u32,
    factor: Item,
}

impl Monkey {
    fn read(str: &str) -> Self {
        let lines: Vec<&str> = str.lines().collect();
        let items: VecDeque<Item> = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|i| i.trim().parse::<Item>().unwrap())
            .collect();
        let operation_str = lines[2].strip_prefix("  Operation: new = old ").unwrap();
        let parts: Vec<&str> = operation_str.split(" ").collect();
        let operation: Box<dyn Fn(Item) -> Item> = match (parts[0], parts[1]) {
            ("*", "old") => Box::new(|old| old * old),
            ("*", n) => {
                let x: Item = n.parse().unwrap();
                Box::new(move |old| old * x)
            }
            ("+", "old") => Box::new(|old| old + old),
            ("+", n) => {
                let x: Item = n.parse().unwrap();
                Box::new(move |old| old + x)
            }
            _ => panic!("unknown operation: {:?}", parts),
        };
        let by = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<Item>()
            .unwrap();
        let test: Box<dyn Fn(Item) -> usize> = {
            let if_true: usize = lines[4]
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap();
            let if_false: usize = lines[5]
                .chars()
                .last()
                .unwrap()
                .to_digit(10)
                .unwrap()
                .try_into()
                .unwrap();
            Box::new(move |n: Item| if n % by == 0 { if_true } else { if_false })
        };
        Self {
            items,
            operation,
            test,
            inspections: 0,
            factor: by,
        }
    }
}

impl std::fmt::Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("inspections", &self.inspections)
            .finish()
    }
}

#[derive(Debug)]
struct State {
    monkeys: Vec<Monkey>,
    has_relief: bool,
    factor: Item,
}

impl State {
    fn read(str: &str, has_relief: bool) -> Self {
        let monkeys: Vec<Monkey> = str.split("\n\n").map(Monkey::read).collect();
        let factor = monkeys.iter().map(|m| m.factor).fold(1, |a, b| a * b);
        println!("Factor is {factor}");
        Self {
            monkeys,
            has_relief,
            factor,
        }
    }

    fn monkey_process(&mut self, idx: usize) {
        while let Some(item) = {
            let monkey = self.monkeys.get_mut(idx).unwrap();
            let i = monkey.items.pop_front();
            if i.is_some() {
                monkey.inspections += 1
            }
            i
        } {
            let monkey = self.monkeys.get(idx).unwrap();
            let item_inspected = (monkey.operation)(item);
            let relieved = if self.has_relief {
                item_inspected / 3
            } else {
                item_inspected % self.factor
            };
            let to = (monkey.test)(relieved);
            let other_monkey = self.monkeys.get_mut(to).unwrap();
            other_monkey.items.push_back(relieved);
            // println!("Monkey {idx} processed item {item} -> {item_inspected} -> {relieved} ==> Sending to {to}")
        }
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            // println!("Processing for Monkey {i}");
            self.monkey_process(i);
        }
    }

    fn monkey_business(&self) -> u64 {
        let mut l: Vec<u32> = self.monkeys.iter().map(|m| m.inspections).collect();
        println!("Inpections: {l:?}");
        l.sort();
        l.iter().rev().take(2).fold(1u64, |a, b| {
            let bx: u64 = (*b).into();
            a * bx
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let content = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
        let total = run1(String::from(content));
        assert_eq!(total, 10605);

        let total = run2(String::from(content));
        assert_eq!(total, 2713310158);
    }
}
