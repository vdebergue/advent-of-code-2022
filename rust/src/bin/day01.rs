use std::env;
use std::fs;

use rust::TopK;
use rust::TopKImpl;

fn main() {
    let args: Vec<String> = env::args().collect();
    let k = args
        .get(1)
        .expect("Should get top k as first number")
        .parse::<usize>()
        .unwrap();
    let filename = args.get(2).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let mut topk: TopKImpl = TopK::new(k);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    contents.split('\n').fold(0, |current, line| {
        if line.is_empty() {
            // end of bag
            topk.add_item(current);
            0
        } else {
            let updated = current + line.trim().parse::<i32>().unwrap();
            updated
        }
    });

    let tops = topk.topk();
    println!("Top k={} is {:?}", k, tops);
    let max = tops.iter().fold(0, |acc, i| acc + i);
    println!("Sum is {}", max)
}
