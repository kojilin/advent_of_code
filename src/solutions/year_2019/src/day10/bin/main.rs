use std::cmp::{max, Ordering};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::error::Error;
use std::fs;

struct Node {
    x: i32,
    y: i32,
    slope: f64,
    in_right_side: bool,
    // include points on same vertical line
    order: i32, // order in same slope
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.order != other.order {
            return self.order.cmp(&other.order);
        }
        if self.in_right_side != other.in_right_side {
            if self.in_right_side {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        if self.slope != other.slope {
            if self.slope > other.slope {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        panic!("impossible")
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/day10/bin/day10.txt")?;
    let lines = input.lines();
    let mut positions: Vec<(i32, i32)> = vec![];
    let mut width = 0;
    let mut height = 0;
    for (y, line) in lines.enumerate() {
        width = line.len();
        height += 1;
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                positions.push((x as i32, y as i32));
            }
        }
    }
    let max_w_h = max(width, height);

    let mut count = 0;
    let mut p = (0, 0);

    for (c_x, c_y) in positions.iter() {
        let mut counts: HashSet<(u64, bool)> = HashSet::new();
        for (t_x, t_y) in positions.iter() {
            if t_x == c_x && t_y == c_y {
                continue;
            }
            let slope = ((c_y - t_y) as f64 / (c_x - t_x) as f64).to_bits();
            let direction = t_x >= c_x;
            counts.insert((slope, direction));
        }
        count = max(count, counts.len());
        if counts.len() == count {
            p = (c_x.clone(), c_y.clone());
        }
    }

    let mut sorted_nodes: BTreeSet<Node> = BTreeSet::new();
    let mut slopes_count: HashMap<(u64, bool), i32> = HashMap::new();

    let (c_x, c_y) = p;

    for i in 1..=max_w_h as i32 {
        for j in c_x - i..=c_x + i {
            if positions.contains(&(j, c_y + i)) {
                let slope = (i as f64 / (j - c_x) as f64);
                let in_right_side = j >= c_x;
                let counter = slopes_count.entry((slope.to_bits(), in_right_side)).or_insert(0);
                sorted_nodes.insert(Node {
                    x: j,
                    y: c_y + i,
                    slope,
                    in_right_side,
                    order: counter.clone(),
                });
                *counter += 1;
            }
            if positions.contains(&(j, c_y - i)) {
                let slope = (-i as f64 / (j - c_x) as f64);
                let in_right_side = j >= c_x;
                let counter = slopes_count.entry((slope.to_bits(), in_right_side)).or_insert(0);
                sorted_nodes.insert(Node {
                    x: j,
                    y: c_y - i,
                    slope,
                    in_right_side,
                    order: counter.clone(),
                });
                *counter += 1;
            }
        }
        for j in c_y - i + 1..c_y + i {
            if positions.contains(&(c_x + i, j)) {
                let slope = ((j - c_y) as f64 / i as f64);
                let in_right_side = true;
                let counter = slopes_count.entry((slope.to_bits(), in_right_side)).or_insert(0);
                sorted_nodes.insert(Node {
                    x: c_x + i,
                    y: j,
                    slope,
                    in_right_side,
                    order: counter.clone(),
                });
                *counter += 1;
            }
            if positions.contains(&(c_x - i, j)) {
                let slope = ((j - c_y) as f64 / -i as f64);
                let in_right_side = false;
                let counter = slopes_count.entry((slope.to_bits(), in_right_side)).or_insert(0);
                sorted_nodes.insert(Node {
                    x: c_x - i,
                    y: j,
                    slope,
                    in_right_side,
                    order: counter.clone(),
                });
                *counter += 1;
            }
        }
    }
    let node = sorted_nodes.iter().nth(199).unwrap();
    println!("{}", node.x * 100 + node.y);
    Ok(())
}

