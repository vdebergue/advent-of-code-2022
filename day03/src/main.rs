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

    let total2 = run2(contents);
    println!("Total part 2 is {total2}");
}

fn run1(contents: String) -> i32 {
    contents
        .lines()
        .filter(|l| !l.is_empty())
        .fold(0, |current, line| {
            let (first, second) = line.split_at(line.len() / 2);
            let first_set = to_set(first);
            let second_set = to_set(second);
            let mut commun = first_set.intersection(&second_set);
            let item = commun.next().unwrap();
            let point = char_to_point(item);
            current + point
        })
}

fn run2(contents: String) -> i32 {
    let v = Vec::from_iter(contents.lines());
    v.chunks(3).fold(0, |current, lines| {
        let group: Vec<HashSet<char>> = lines.iter().map(|l| to_set(l)).collect();
        let common = group.iter().skip(1).fold(group[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });
        let group = common.iter().next().unwrap();
        current + char_to_point(group)
    })
}

fn to_set(l: &str) -> HashSet<char> {
    HashSet::from_iter(l.chars())
}

fn char_to_point(c: &char) -> i32 {
    if c.is_ascii_lowercase() {
        1 + *c as i32 - 'a' as i32
    } else {
        1 + 26 + *c as i32 - 'A' as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        assert_eq!(char_to_point(&'a'), 1);
        assert_eq!(char_to_point(&'b'), 2);
        assert_eq!(char_to_point(&'A'), 27);
        assert_eq!(char_to_point(&'L'), 38);
        assert_eq!(char_to_point(&'P'), 42);
    }

    #[test]
    fn sample() {
        let content = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let total = run1(String::from(content));
        assert_eq!(total, 157)
    }
}
