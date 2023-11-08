use std::collections::HashSet;
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

    let out = run2(contents.clone());
    println!("{}", out);
}

fn run1(contents: String) -> i32 {
    let cycle_count: HashSet<i32> = HashSet::from_iter([20, 60, 100, 140, 180, 220]);
    let mut total = 0;
    contents
        .lines()
        .filter(|l| !l.is_empty())
        .fold(State::init(), |state, line| {
            let inst = Instruction::read(line);
            state.exec(inst, |state: &State| {
                if cycle_count.contains(&state.cycle) {
                    total += state.signal_strength();
                    // println!("{state:?} : {}, {}", state.signal_strength(), total);
                }
            })
        });

    total
}

fn run2(contents: String) -> String {
    let mut crt = CRT::new(240);
    contents
        .lines()
        .filter(|l| !l.is_empty())
        .fold(State::init(), |state, line| {
            let inst = Instruction::read(line);
            state.exec(inst, |state: &State| {
                crt.draw(state);
            })
        });

    crt.display()
}

#[derive(Debug)]
struct State {
    x: i32,
    cycle: i32,
}

impl State {
    fn init() -> Self {
        State { x: 1, cycle: 1 }
    }
    fn incr_cycle(&self) -> Self {
        Self {
            x: self.x,
            cycle: self.cycle + 1,
        }
    }
    fn add_x(&self, dx: i32) -> Self {
        Self {
            x: self.x + dx,
            cycle: self.cycle,
        }
    }

    fn signal_strength(&self) -> i32 {
        self.x * self.cycle
    }

    fn exec(&self, instruction: Instruction, mut on_cycle: impl FnMut(&State) -> ()) -> Self {
        match instruction {
            Instruction::Noop => {
                on_cycle(self);
                let new_state = self.incr_cycle();
                new_state
            }
            Instruction::AddX { dx } => {
                on_cycle(self);
                let s1 = self.incr_cycle();
                on_cycle(&s1);
                let s2 = s1.incr_cycle().add_x(dx);
                s2
            }
        }
    }
}

enum Instruction {
    Noop,
    AddX { dx: i32 },
}

impl Instruction {
    fn read(str: &str) -> Self {
        if str.starts_with("addx") {
            let dx = str.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();
            Instruction::AddX { dx }
        } else if str == "noop" {
            Instruction::Noop
        } else {
            panic!("Invalid instruction {}", str.to_string())
        }
    }
}

struct CRT {
    pixels: Vec<char>,
}
impl CRT {
    fn new(size: usize) -> Self {
        Self {
            pixels: vec![' '; size],
        }
    }

    fn draw(&mut self, state: &State) {
        let vertical_position = (state.cycle - 1) % 40;
        let char = if state.x - 1 <= vertical_position && vertical_position <= state.x + 1 {
            '#'
        } else {
            '.'
        };
        let i: usize = state.cycle.try_into().unwrap();
        // if state.cycle <= 40 {
        //     println!("{state:?} {vertical_position:?} {char:?} position={i}");
        // }
        self.pixels[i - 1] = char;
    }

    fn display(&self) -> String {
        let parts: Vec<String> = self
            .pixels
            .chunks(40)
            .map(|l| {
                let s: String = l.iter().collect();
                s
            })
            .collect();
        parts.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let content = "
noop
addx 3
addx -5
";
        content
            .lines()
            .filter(|l| !l.is_empty())
            .fold(State::init(), |state, line| {
                let inst = Instruction::read(line);
                state.exec(inst, |state: &State| {
                    println!("{state:?}");
                })
            });
    }

    #[test]
    fn sample() {
        let content = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        let total = run1(String::from(content));
        assert_eq!(total, 13140);

        let out = run2(String::from(content));
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        let out_lines: Vec<&str> = out.lines().collect();
        expected.lines().enumerate().for_each(|(idx, line)| {
            println!("Line {idx}:\n{}\n{}", out_lines[idx], line);
            assert_eq!(out_lines[idx], line);
        });
        assert_eq!(out, expected);
    }
}
