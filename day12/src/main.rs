use std::env;
use std::fs;

extern crate pathfinding;
use pathfinding::prelude::astar;

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

fn run1(contents: &str) -> i64 {
    let map = Map::read(contents);
    // println!("{map:?}");
    let out = astar(
        &map.start,
        |p| {
            map.possible_neighbors(p)
                .into_iter()
                .map(|p| (p, 1))
                .collect::<Vec<(Position, i64)>>()
        },
        |p| map.distance_to_end(p),
        |p| *p == map.end,
    );
    println!("{out:?}");
    out.unwrap().1
}

fn run2(contents: &str) -> i64 {
    let map = Map::read(contents);
    std::iter::once(&map.start)
        .chain(map.low_points.iter())
        .flat_map(|start| {
            let out = astar(
                start,
                |p| {
                    map.possible_neighbors(p)
                        .into_iter()
                        .map(|p| (p, 1))
                        .collect::<Vec<(Position, i64)>>()
                },
                |p| map.distance_to_end(p),
                |p| *p == map.end,
            );
            out.map(|o| o.1)
        })
        .min()
        .unwrap()
}

type Position = (usize, usize);
#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<u32>>,
    start: Position,
    end: Position,
    low_points: Vec<Position>,
}

impl Map {
    fn read(str: &str) -> Self {
        let mut start: Position = (0, 0);
        let mut end: Position = (0, 0);
        let mut low_points: Vec<Position> = Vec::new();
        let tiles: Vec<Vec<u32>> = str
            .lines()
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(y, l)| {
                let row: Vec<u32> = l
                    .chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        'S' => {
                            start = (x, y);
                            char_to_point(&'a')
                        }
                        'E' => {
                            end = (x, y);
                            char_to_point(&'z')
                        }
                        'a' => {
                            low_points.push((x, y));
                            char_to_point(&'a')
                        }
                        _ => char_to_point(&c),
                    })
                    .collect();

                row
            })
            .collect();
        Self {
            tiles,
            start,
            end,
            low_points,
        }
    }

    fn get(&self, pos: &Position) -> u32 {
        self.tiles[pos.1][pos.0]
    }

    fn possible_neighbors(&self, pos: &Position) -> Vec<Position> {
        let (x, y) = *pos;
        let curr_height = self.get(pos);
        let (xi, yi) = (x as i64, y as i64);
        let neighbors: Vec<Position> = vec![(xi + 1, yi), (xi - 1, yi), (xi, yi + 1), (xi, yi - 1)]
            .into_iter()
            .filter(|p| {
                let (x, y) = *p;
                x >= 0
                    && y >= 0
                    && (y as usize) < self.tiles.len()
                    && (x as usize) < self.tiles[y as usize].len()
            })
            .map(|p| (p.0 as usize, p.1 as usize))
            .collect();
        let possible_neighbors: Vec<Position> = neighbors
            .into_iter()
            .filter(|n| {
                let height = self.get(&n);
                height == curr_height + 1 || height <= curr_height
            })
            .collect();
        possible_neighbors
    }

    fn distance_to_end(&self, pos: &Position) -> i64 {
        (pos.0 as i64 - self.end.0 as i64).abs() + (pos.1 as i64 - self.end.1 as i64).abs()
    }
}

fn char_to_point(c: &char) -> u32 {
    *c as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
        ";

        assert_eq!(run1(input), 31);
    }
}
