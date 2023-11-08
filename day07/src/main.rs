use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args.get(1).expect("Args should contain a filename");
    println!("Args: {:?}", args);
    println!("Reading file {}", filename);

    let contents = fs::read_to_string(filename).expect("Should have been able to read the file");

    let root = read_fs(contents);
    println!("Total is {}", root.total_size());
    let total1 = run1(root);
    println!("Total part1 is {}", total1);
    // println!(
    //     "Total with size {}",
    //     out.iter().map(|n| n.total_size()).fold(0, |a, x| a + x)
    // );
}

fn run1(root: Node) -> usize {
    // let mut out = Vec::new();
    // walk(&root, &mut out, |n| n.total_size() < 100000);
    // out.iter().map(|n| n.total_size()).sum()
    let totals = get_all_totals_from(&root);
    println!("{:?}", totals);
    totals.iter().filter(|&t| t <= &100000).sum()
}

fn read_fs(contents: String) -> Node {
    let mut root = Node {
        name: String::new(),
        size: 0,
        children: HashMap::new(),
    };
    let mut current: &mut Node = &mut root;

    let mut parents: Vec<String> = Vec::new();

    let lines = contents.lines().filter(|l| !l.is_empty());
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "$" {
            let command = parts[1];
            if command == "cd" {
                let arg = parts[2];
                if arg == ".." {
                    current = parents.pop().unwrap();
                } else {
                    if !current.children.contains_key(arg) {
                        let new_node = Node {
                            name: arg.to_string(),
                            size: 0,
                            children: HashMap::new(),
                        };
                        current.children.insert(new_node.name.clone(), new_node);
                    }
                    let node = current.children.get_mut(arg).unwrap();
                    parents.push(arg.to_string());
                    current = node;
                }
            } else if command == "ls" {
                // do nothing
            }
        } else {
            // doing a ls
            if parts[0] == "dir" {
                // skip
            } else {
                let size = parts[0];
                let name = String::from(parts[1]);
                let node = Node {
                    name: name.clone(),
                    size: size.parse::<usize>().unwrap(),
                    children: HashMap::new(),
                };
                current.children.insert(name, node);
            }
        }
    }
    root
}

// type Predicate<T> = fn(&T) -> bool;
// fn walk<'a>(root: &'a Node, out: &'a mut Vec<&'a Node>, pred: Predicate<Node>) {
//     if pred(root) {
//         out.push(root)
//     }
//     root.children
//         .iter()
//         .for_each(|(_, child)| walk(child, out, pred));
// }

fn get_all_totals_from(system: &Node) -> Vec<usize> {
    let mut sizes = vec![];
    get_total_sizes(system, &mut sizes);
    sizes
}

fn get_total_sizes(system: &Node, mut vec: &mut Vec<usize>) {
    system
        .children
        .iter()
        .for_each(|(_, dir)| get_total_sizes(&dir, &mut vec));

    println!("Size of {} is {}", system.name, system.total_size());
    vec.push(system.total_size())
}

struct Node {
    name: String,
    size: usize,
    children: HashMap<String, Node>,
}

impl Node {
    fn total_size(&self) -> usize {
        // The size of a directory is the sum of the sizes of its children
        let mut size = self.size;
        for (_, child) in &self.children {
            size += child.total_size();
        }
        size
    }

    fn get_mut(&mut self, path: &Vec<String>) -> &mut Self {
        match path.get(0) {
            Some(p) => self.children.get_mut(p).unwrap().get_mut(path[1..]),
            None => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let content = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
        let node = read_fs(String::from(content));
        assert_eq!(node.total_size(), 48381165);
        assert_eq!(run1(node), 95437);
    }
}
