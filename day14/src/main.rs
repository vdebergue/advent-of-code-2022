use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let total = run1(&contents);
    println!("Total is {total}");

    let total2 = run2(&contents);
    println!("Total part 2 is {total2}");
}

fn run1(contents: &str) -> usize {}

type Position = (usize, usize);

struct State {
    width: usize,
    height: usize,
    items: Vec<Items>,
    start: Position,
}

impl State {
    fn read(str: &str) -> Self {
        let rock_structures: Vec<Vec<Position>> = str
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| {
                let rocks = line
                    .split(" -> ")
                    .map(|p| {
                        let xy: Vec<usize> =
                            p.split(',').map(|c| c.parse::<usize>().unwrap()).collect();
                        (xy[0], xy[1])
                    })
                    .collect();
                rocks
            })
            .collect();
        let mut_

        Self {}
    }
}

enum Items {
    Rock,
    Sand,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

        assert_eq!(run1(input), 31);
    }
}
