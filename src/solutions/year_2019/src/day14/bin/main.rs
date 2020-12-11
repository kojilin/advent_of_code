use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::ops::DerefMut;

struct Node {
    name: String,
    output_count: i64,
    outgoing_count: i64,
    incoming: Vec<Edge>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.incoming.len().cmp(&other.incoming.len())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn create_node(name: String, output_count: i64) -> Node {
        Node {
            name,
            output_count,
            outgoing_count: 0,
            incoming: vec![],
        }
    }

    fn build_connection(&mut self, to: &mut Node, require_count: i64) {
        self.outgoing_count += 1;
        to.incoming.push(Edge {
            from: self.name.clone(),
            require_count,
        });
    }
}

struct Edge {
    from: String,
    require_count: i64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut min = 1185930;
    let mut max = 2371860;

    while min < max {
        let middle = (max - min) / 2 + min;
        let tmp = calculate(middle)?;
        if tmp > 1000000000000 {
            max = middle;
        } else {
            min = middle + 1;
        }
    }
    println!("{}", min - 1);
    Ok(())
}

fn calculate(count: i64) -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/day14/bin/day14.txt")?;
    let mut nodes: HashMap<String, RefCell<Node>> = HashMap::new();
    nodes.insert(String::from("ORE"), RefCell::new(Node::create_node(String::from("ORE"), 1)));
    let mut formulas: HashMap<String, HashMap<String, i64>> = HashMap::new();

    for line in input.lines() {
        let from_to: Vec<&str> = line.split("=>").collect();
        let pair: Vec<&str> = from_to[1].trim().split(' ').collect();
        nodes.insert(pair[1].to_string(), RefCell::new(Node::create_node(pair[1].to_string(),
                                                                         pair[0].parse::<i64>().unwrap())));
        let left: HashMap<String, i64> = from_to[0].split(", ")
            .map(|s| s.trim())
            .map(|s| {
                let pair: Vec<&str> = s.split(' ').collect();
                (pair[1].to_string(), pair[0].parse::<i64>().unwrap())
            })
            .collect();
        formulas.insert(pair[1].to_string(), left);
    }

    for relation in formulas {
        for from_and_count in relation.1 {
            let to = nodes.get(&relation.0).unwrap();
            let from = nodes.get(&from_and_count.0).unwrap();
            from.borrow_mut().build_connection(to.borrow_mut().deref_mut(), from_and_count.1);
        }
    }

    let mut needed_count: HashMap<String, i64> = HashMap::new();
    let mut remain: Vec<String> = vec![];
    needed_count.insert(String::from("FUEL"), count);
    remain.push(String::from("FUEL"));
    while remain.len() > 0 {
        let to = nodes.remove(&remain.remove(0)).unwrap().into_inner();
        let count = needed_count.get(&to.name).unwrap().clone();
        for edge in to.incoming {
            let from_name = &edge.from;
            let multiplier = (count - 1) / to.output_count + 1;
            let total = multiplier * edge.require_count;
            let count = needed_count.entry(String::from(from_name)).or_insert(0);
            *count += total;
            let mut from = nodes.get_mut(from_name).unwrap().borrow_mut();
            from.outgoing_count = from.outgoing_count - 1;
            if from.outgoing_count == 0 {
                remain.push(from.name.clone());
            }
        }
    }
    Ok(needed_count.get("ORE").unwrap().clone())
}
