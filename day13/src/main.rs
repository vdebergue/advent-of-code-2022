use std::cmp::Ordering;
use std::env;
use std::fs;

extern crate serde_json;
use serde_json::Value;

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

fn run1(input: &str) -> usize {
    input
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .enumerate()
        .map(|(idx, pair_str)| {
            let pair: Vec<Value> = pair_str
                .split("\n")
                .filter(|l| !l.is_empty())
                .map(|l| serde_json::from_str(l).unwrap())
                .collect();
            // println!("{pair:?}");
            if compare(&pair[0], &pair[1]) == Ordering::Less {
                idx + 1
            } else {
                0
            }
        })
        .sum()
}

fn run2(input: &str) -> usize {
    let mut packets: Vec<Value> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let d1: Value = serde_json::from_str("[[2]]").unwrap();
    let d2: Value = serde_json::from_str("[[6]]").unwrap();
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort_by(compare);
    packets
        .iter()
        .enumerate()
        .map(|(idx, packet)| {
            if packet == &d1 || packet == &d2 {
                idx + 1
            } else {
                1
            }
        })
        .fold(1, |a, b| a * b)
}

fn compare(left: &Value, right: &Value) -> Ordering {
    // println!("--- {left} ? {right}--- ");
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            let (lx, rx) = (l.as_i64().unwrap(), r.as_i64().unwrap());
            if lx == rx {
                Ordering::Equal
            } else if lx < rx {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        (Value::Number(l), Value::Array(_)) => {
            let n = serde_json::Number::from(l.as_i64().unwrap());
            compare(&Value::Array(vec![Value::Number(n)]), right)
        }
        (Value::Array(_), Value::Number(r)) => {
            let n = serde_json::Number::from(r.as_i64().unwrap());
            compare(left, &Value::Array(vec![Value::Number(n)]))
        }
        (Value::Array(l), Value::Array(r)) if l.is_empty() && r.is_empty() => Ordering::Equal,
        (Value::Array(l), Value::Array(r)) if l.is_empty() && !r.is_empty() => Ordering::Less,
        (Value::Array(l), Value::Array(r)) if !l.is_empty() && r.is_empty() => Ordering::Greater,
        (Value::Array(l), Value::Array(r)) => {
            // println!("Comparing {:?} and {:?}", l, r);
            for i in 0..l.len() {
                if i >= r.len() {
                    return Ordering::Greater;
                }
                let outi = compare(&l[i], &r[i]);
                // println!("Compare {} and {} => {outi:?}", l[i], r[i]);
                match outi {
                    Ordering::Equal => continue,
                    _ => return outi,
                }
            }
            return Ordering::Less;
        }
        _ => panic!("Unhandled"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
        assert_eq!(run1(input), 13);
    }
}
