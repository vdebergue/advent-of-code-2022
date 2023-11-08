use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let total = run(contents.clone(), 2);
    println!("Total is {total}");

    let total2 = run(contents.clone(), 10);
    println!("Total part 2 is {total2}");
}

fn run(contents: String, length: usize) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut state = Rope {
        knots: vec![Position { x: 0, y: 0 }; length],
    };
    contents.lines().filter(|l| !l.is_empty()).for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let dir = parts[0];
        let steps = parts[1].parse::<usize>().unwrap();
        println!("{dir} {steps}");
        for _ in 0..steps {
            state = state.step(dir);
            visited.insert(state.tail());
            // println!("{state:?}");
        }
    });
    visited.len()
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    fn head(&self) -> Position {
        self.knots[0]
    }
    fn tail(&self) -> Position {
        self.knots[self.knots.len() - 1]
    }
    fn step(&mut self, dir: &str) -> Self {
        let new_head = self.head().step(dir);
        let mut leader = new_head;
        let mut new_knots: Vec<Position> = Vec::with_capacity(self.knots.len());
        new_knots.push(new_head);
        for i in 1..self.knots.len() {
            let k = self.knots[i];
            let new_k = k.follow_head(leader);
            leader = new_k;
            new_knots.push(new_k);
        }
        Rope { knots: new_knots }
    }
}

impl Position {
    fn step(&self, dir: &str) -> Self {
        match dir {
            "U" => Position {
                x: self.x,
                y: self.y + 1,
            },
            "D" => Position {
                x: self.x,
                y: self.y - 1,
            },
            "R" => Position {
                x: self.x + 1,
                y: self.y,
            },
            "L" => Position {
                x: self.x - 1,
                y: self.y,
            },
            _ => todo!(),
        }
    }

    fn follow_head(&self, head: Self) -> Self {
        let dx = (head.x - self.x).abs();
        let dy = (head.y - self.y).abs();
        if dx < 2 && dy < 2 {
            *self
        } else {
            let dx_sig = (head.x - self.x).signum();
            let dy_sig = (head.y - self.y).signum();
            Position {
                x: self.x + dx_sig,
                y: self.y + dy_sig,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let content = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
        let total = run(String::from(content), 2);
        assert_eq!(total, 13);

        let totalb = run(String::from(content), 9);
        assert_eq!(totalb, 1)
    }
}
