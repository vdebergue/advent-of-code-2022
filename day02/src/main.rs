use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let total1 = run1(contents.clone());
    println!("Total Part1 score is: {total1}");

    let total2 = run2(contents);
    println!("Total Part2 score is: {total2}");
}

fn run1(contents: String) -> i32 {
    contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .fold(0, |current, line| {
            let parts = Vec::from_iter(line.trim().split(' '));
            let my_move = Move::read(parts[1]).unwrap();
            let other = Move::read(parts[0]).unwrap();
            let move_value = my_move.value();
            let result = Outcome::get(my_move, other);
            let round_score = result.value() + move_value;
            current + round_score
        })
}

fn run2(contents: String) -> i32 {
    contents
        .split('\n')
        .filter(|l| !l.is_empty())
        .fold(0, |current, line| {
            let parts = Vec::from_iter(line.trim().split(' '));
            let other = Move::read(parts[0]).unwrap();
            let outcome = Outcome::read(parts[1]).unwrap();

            let my_move = Outcome::get_move(other, outcome);
            let move_value = my_move.value();
            let round_score = outcome.value() + move_value;
            current + round_score
        })
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn value(&self) -> i32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn read(chars: &str) -> Result<Self, String> {
        match chars {
            "A" => Result::Ok(Move::Rock),
            "B" => Result::Ok(Move::Paper),
            "C" => Result::Ok(Move::Scissors),
            "X" => Result::Ok(Move::Rock),
            "Y" => Result::Ok(Move::Paper),
            "Z" => Result::Ok(Move::Scissors),
            other => Result::Err(format!("Could not read move from '{other}'")),
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn value(&self) -> i32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }

    fn get(my: Move, other: Move) -> Self {
        if my == other {
            Outcome::Draw
        } else if my == Move::Paper && other == Move::Rock {
            Outcome::Win
        } else if my == Move::Rock && other == Move::Scissors {
            Outcome::Win
        } else if my == Move::Scissors && other == Move::Paper {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    fn read(chars: &str) -> Result<Self, String> {
        match chars {
            "X" => Result::Ok(Self::Lose),
            "Y" => Result::Ok(Self::Draw),
            "Z" => Result::Ok(Self::Win),
            other => Result::Err(format!("Could not read Outcome from {other}")),
        }
    }

    fn get_move(other: Move, outcome: Self) -> Move {
        if outcome == Self::Draw {
            other.clone()
        } else if outcome == Self::Lose {
            match other {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            }
        } else {
            match other {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Import the code I want to test
    use super::*;
    // My tests
    #[test]
    fn sample() {
        let content = String::from(
            "
A Y
B X
C Z",
        );
        let result = run1(content.clone());
        assert_eq!(result, 15);

        let result2 = run2(content);
        assert_eq!(result2, 12);
    }
}
