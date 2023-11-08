use std::env;
use std::fs;

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

pub trait TopK {
    fn new(k: usize) -> Self;
    fn add_item(&mut self, item: i32);

    fn topk(&self) -> Vec<i32>;
}

pub struct TopKImpl {
    items: Vec<i32>,
}

impl TopK for TopKImpl {
    fn new(k: usize) -> Self {
        Self { items: vec![0; k] }
    }

    fn add_item(&mut self, item: i32) {
        let length = self.items.len();
        for index in 0..length {
            let i = self.items[index];
            if item > i {
                println!(
                    "Adding item {} to {:?} at index {}",
                    item, self.items, index
                );
                for j in (index + 1..length).rev() {
                    self.items[j] = self.items[j - 1];
                }
                self.items[index] = item;
                break;
            }
        }
    }

    fn topk(&self) -> Vec<i32> {
        self.items.clone()
    }
}
