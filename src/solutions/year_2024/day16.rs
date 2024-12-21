use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fmt::Formatter;
use std::io::Write;
use std::{fmt, fs};
use Facing::{Down, Left, Right, Up};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Facing {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Up => {
                write!(f, "Up")
            }
            Down => {
                write!(f, "Down")
            }
            Left => {
                write!(f, "Left")
            }
            Right => {
                write!(f, "Right")
            }
        }
    }
}

fn solve_day16() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day16.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut start = (-1, -1);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (y as i64, x as i64);
                break;
            }
        }
    }

    Ok(bfs(start, &map))
}

#[derive(Debug, Hash, Clone)]
struct Score {
    y: i64,
    x: i64,
    facing: Facing,
    cost: i64,
}

impl Score {
    pub fn new(y: i64, x: i64, facing: Facing, cost: i64) -> Self {
        Self { y, x, facing, cost }
    }
}

impl PartialEq<Self> for Score {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && self.y == other.y
            && self.x == other.x
            && self.facing == other.facing
    }
}

impl Eq for Score {}

impl PartialOrd<Self> for Score {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            .then(self.y.cmp(&other.y))
            .then(self.x.cmp(&other.x))
            .then(self.facing.cmp(&other.facing))
    }
}

fn bfs(current: (i64, i64), map: &Vec<Vec<char>>) -> i64 {
    let mut queue: BTreeSet<Score> = BTreeSet::new();
    // y, x, facing
    let mut visited: HashSet<(i64, i64, Facing)> = HashSet::new();
    // Up, Right, Down, Left
    const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    queue.insert(Score::new(current.0, current.1, Facing::Right, 0));
    while !queue.is_empty() {
        let pop = queue.pop_first().unwrap();
        // println!("{:?}", pop);
        if visited.contains(&(pop.y, pop.x, pop.facing)) {
            continue;
        }
        if map[pop.y as usize][pop.x as usize] == 'E' {
            return pop.cost;
        }
        visited.insert((pop.y, pop.x, pop.facing));

        for i in 0..4 {
            let next = (pop.y + DIRECTION[i].0, pop.x + DIRECTION[i].1);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= map.len() as i64
                || next.1 >= map[0].len() as i64
            {
                continue;
            }
            if map[next.0 as usize][next.1 as usize] == '#' {
                continue;
            }

            // Up
            if i == 0 {
                if matches!(pop.facing, Up) {
                    queue.insert(Score::new(next.0, next.1, Facing::Up, pop.cost + 1));
                } else if matches!(pop.facing, Left) || matches!(pop.facing, Right) {
                    queue.insert(Score::new(next.0, next.1, Facing::Up, pop.cost + 1 + 1000));
                }
            } else if i == 1 {
                if matches!(pop.facing, Right) {
                    queue.insert(Score::new(next.0, next.1, Facing::Right, pop.cost + 1));
                } else if matches!(pop.facing, Up) || matches!(pop.facing, Down) {
                    queue.insert(Score::new(
                        next.0,
                        next.1,
                        Facing::Right,
                        pop.cost + 1 + 1000,
                    ));
                }
            } else if i == 2 {
                if matches!(pop.facing, Down) {
                    queue.insert(Score::new(next.0, next.1, Facing::Down, pop.cost + 1));
                } else if matches!(pop.facing, Left) || matches!(pop.facing, Right) {
                    queue.insert(Score::new(
                        next.0,
                        next.1,
                        Facing::Down,
                        pop.cost + 1 + 1000,
                    ));
                }
            } else if i == 3 {
                if matches!(pop.facing, Left) {
                    queue.insert(Score::new(next.0, next.1, Facing::Left, pop.cost + 1));
                } else if matches!(pop.facing, Up) || matches!(pop.facing, Down) {
                    queue.insert(Score::new(
                        next.0,
                        next.1,
                        Facing::Left,
                        pop.cost + 1 + 1000,
                    ));
                }
            }
        }
    }
    -1
}

#[derive(Debug, Hash, Clone)]
struct ScoreWithPrev {
    score: Score,
    prev: Option<Score>,
}

impl ScoreWithPrev {
    pub fn new(score: Score, prev: Option<Score>) -> Self {
        Self { score, prev }
    }
}

impl PartialEq<Self> for ScoreWithPrev {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.prev == other.prev
    }
}

impl Eq for ScoreWithPrev {}

impl PartialOrd<Self> for ScoreWithPrev {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScoreWithPrev {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.prev.cmp(&other.prev))
    }
}

fn solve_day16_2() -> Result<i64, Box<dyn Error>> {
    let input = fs::read_to_string("src/solutions/year_2024/day16.txt")?;
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut start = (-1, -1);

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                start = (y as i64, x as i64);
                break;
            }
        }
    }

    // key is position and direction
    Ok(bfs_2(start, &map))
}

fn bfs_2(start_point: (i64, i64), map: &Vec<Vec<char>>) -> i64 {
    let mut queue: BTreeSet<ScoreWithPrev> = BTreeSet::new();
    let mut previous: HashMap<Score, Vec<Score>> = HashMap::new();
    // to know current min cost of map
    let mut costs: HashMap<Score, i64> = HashMap::new();
    // y, x, facing
    let mut visited: HashSet<(i64, i64, Facing)> = HashSet::new();
    // Up, Right, Down, Left
    const DIRECTION: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    queue.insert(ScoreWithPrev::new(
        Score::new(start_point.0, start_point.1, Facing::Right, 0),
        None,
    ));
    let mut trace = None;
    while !queue.is_empty() {
        // println!("previous: {:?}", previous);
        // println!("costs: {:?}", costs);
        let pop = queue.pop_first().unwrap();
        if let Some(&existing) = costs.get(&pop.score) {
            if pop.score.cost == existing {
                // put to the previous
                if let Some(previous) = previous.get_mut(&pop.score) {
                    previous.push(pop.prev.as_ref().unwrap().clone());
                } else {
                    panic!("should not happen");
                }
            }
        }
        // println!("{:?}", pop);
        if visited.contains(&(pop.score.y, pop.score.x, pop.score.facing)) {
            continue;
        }
        // it should be the smallest cost
        if let Some(existing) = costs.insert(pop.score.clone(), pop.score.cost) {
            if pop.score.cost < existing {
                panic!("should not happen");
            }
        }
        if let Some(prev) = pop.prev.as_ref() {
            previous.insert(pop.score.clone(), vec![prev.clone()]);
        }

        if map[pop.score.y as usize][pop.score.x as usize] == 'E' {
            trace = Some(pop.score.clone());
            break;
        }
        visited.insert((pop.score.y, pop.score.x, pop.score.facing));

        for dir_index in 0..4 {
            let next = (
                pop.score.y + DIRECTION[dir_index].0,
                pop.score.x + DIRECTION[dir_index].1,
            );
            if next.0 < 0
                || next.1 < 0
                || next.0 >= map.len() as i64
                || next.1 >= map[0].len() as i64
            {
                continue;
            }
            if map[next.0 as usize][next.1 as usize] == '#' {
                continue;
            }

            // Up
            if dir_index == 0 {
                if matches!(pop.score.facing, Up) {
                    queue.insert(ScoreWithPrev::new(
                        Score::new(next.0, next.1, Facing::Up, pop.score.cost + 1),
                        Some(pop.score.clone()),
                    ));
                } else if matches!(pop.score.facing, Left) || matches!(pop.score.facing, Right) {
                    // just turn
                    queue.insert(
                        (ScoreWithPrev::new(
                            Score::new(pop.score.y, pop.score.x, Facing::Up, pop.score.cost + 1000),
                            Some(pop.score.clone()),
                        )),
                    );
                }
            } else if dir_index == 1 {
                if matches!(pop.score.facing, Right) {
                    queue.insert(ScoreWithPrev::new(
                        Score::new(next.0, next.1, Facing::Right, pop.score.cost + 1),
                        Some(pop.score.clone()),
                    ));
                } else if matches!(pop.score.facing, Up) || matches!(pop.score.facing, Down) {
                    // just turn
                    queue.insert(
                        (ScoreWithPrev::new(
                            Score::new(
                                pop.score.y,
                                pop.score.x,
                                Facing::Right,
                                pop.score.cost + 1000,
                            ),
                            Some(pop.score.clone()),
                        )),
                    );
                }
            } else if dir_index == 2 {
                if matches!(pop.score.facing, Down) {
                    queue.insert(ScoreWithPrev::new(
                        Score::new(next.0, next.1, Facing::Down, pop.score.cost + 1),
                        Some(pop.score.clone()),
                    ));
                } else if matches!(pop.score.facing, Right) || matches!(pop.score.facing, Left) {
                    // just turn
                    queue.insert(
                        (ScoreWithPrev::new(
                            Score::new(
                                pop.score.y,
                                pop.score.x,
                                Facing::Down,
                                pop.score.cost + 1000,
                            ),
                            Some(pop.score.clone()),
                        )),
                    );
                }
            } else if dir_index == 3 {
                if matches!(pop.score.facing, Left) {
                    queue.insert(ScoreWithPrev::new(
                        Score::new(next.0, next.1, Facing::Left, pop.score.cost + 1),
                        Some(pop.score.clone()),
                    ));
                } else if matches!(pop.score.facing, Up) || matches!(pop.score.facing, Down) {
                    //just turn
                    queue.insert(
                        (ScoreWithPrev::new(
                            Score::new(
                                pop.score.y,
                                pop.score.x,
                                Facing::Left,
                                pop.score.cost + 1000,
                            ),
                            Some(pop.score.clone()),
                        )),
                    );
                }
            }
        }
    }

    let mut result = HashSet::new();
    let target = Score::new(start_point.0, start_point.1, Right, 0);
    // it's from right
    let mut queue = VecDeque::new();
    queue.push_back(trace.unwrap());
    while let Some(now) = queue.pop_front() {
        result.insert((now.y, now.x));
        if let Some(prev) = previous.get(&now) {
            for next in prev {
                queue.push_back(next.clone());
            }
        }
    }
    result.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_1() {
        println!("result: {:?}", solve_day16());
    }

    #[test]
    fn test_day16_2() {
        println!("result: {:?}", solve_day16_2());
    }
}
