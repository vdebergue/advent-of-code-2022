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
