use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let total = run(contents.clone(), 4);
    println!("Total is {total}");

    let total2 = run(contents, 14);
    println!("Total part 2 is {total2}");
}

fn run(contents: String, packet_size: usize) -> usize {
    for i in 0..(contents.len() - packet_size) {
        let p = &contents[i..(i + packet_size)];
        let chars: HashSet<char> = HashSet::from_iter(p.chars());
        if chars.len() == packet_size {
            return i + packet_size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(run(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4), 5);
        assert_eq!(run(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4), 6);
        assert_eq!(
            run(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4),
            10
        );

        assert_eq!(run(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 14), 23);
        assert_eq!(run(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 14), 23);
        assert_eq!(
            run(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 14),
            29
        );
    }
}
