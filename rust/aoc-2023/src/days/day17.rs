use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Node {
    r: i32,
    c: i32,
    d: char,
    history: (usize, usize, usize, usize), // (Up, Right, Down, Left)
    cost: i32,
    heuristic: i32,
    path: Vec<(i32, i32, char)>
}

impl Eq for Node {}

impl PartialEq<Self> for Node {
    fn eq(&self, other: &Self) -> bool {
        if self.r.eq(&other.r)
            && self.c.eq(&other.c)
            && self.d.eq(&other.d)
            && self.history.eq(&other.history)
        {
            return true;
        }
        false
    }
}
impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.c.hash(state);
        self.d.hash(state);
        self.history.hash(state);
    }
}

impl Ord for Node{
    fn cmp(&self, other: &Self) -> Ordering {
        match (other.cost + other.heuristic) - (&self.cost + &self.heuristic){
            res if res > 0 => Ordering::Greater,
            res if res == 0 => Ordering::Equal,
            res if res < 0 => Ordering::Less,
            _ => panic!("Could not compare Nodes")
        }
    }
}

fn heuristic(row:i32, col: i32, grid: &Vec<Vec<i32>>) -> i32 {
    // Uncomment for A*
    // grid.len() as i32 - row + grid[0].len() as i32 - col
    0
}

fn path_cost(path: Vec<(i32, i32, char)>, grid: &Vec<Vec<i32>>) -> i32 {
    let mut cost = 0;
    for i in 0..path.len() {
        cost += grid[path[i].0 as usize][path[i].1 as usize]
    }
    cost - grid[0][0] + grid[grid.len() - 1][grid[0].len() - 1]
}

fn add_node(node:Node, heap: &mut BinaryHeap<Node>, seen: &HashSet<Node>) {
    if seen.contains(&node) {
        return
    }
    // Path must not go in the same direction more than 3 times
    let steps = 2;
    if node.history.0 > steps || node.history.1 > steps || node.history.2 > steps || node.history.3 > steps {
        return
    }
    heap.push(node);
}

pub fn execute(input: Vec<String>) -> (Option<String>, Option<String>) {
    let grid:Vec<Vec<i32>> = input.iter().map(|r| r.chars()
        .map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut seen: HashSet<Node> = HashSet::new();
    add_node(Node {
        r: 0, c: 1, d: '>', history: (0,1,0,0), cost: 0, heuristic: heuristic(0, 1, &grid), path: vec![(0,0,'>')]
    }, &mut heap, &seen);
    add_node(Node {
        r: 1, c: 0, d: 'v', history: (0,0,1,0), cost: 0, heuristic: heuristic(1, 0, &grid), path: vec![(0,0,'v')]
    }, &mut heap, &seen);
    let mut node = heap.pop().unwrap();
    while node.r != (input.len() - 1) as i32 || node.c != (input[0].len() - 1) as i32 {
        if seen.contains(&node) {
            seen.insert(node.clone());
            node = heap.pop().unwrap();
            continue;
        }
        // Path must go through grid
        if !(0 <= node.r && node.r < grid.len() as i32 && 0 <= node.c && node.c < grid[0].len() as i32) {
            seen.insert(node.clone());
            node = heap.pop().unwrap();
            continue;
        }
        let cost = node.cost + grid[node.r as usize][node.c as usize];
        let mut path: Vec<(i32, i32, char)> = node.path.clone();
        path.push((node.r, node.c, node.d));
        match node {
            Node {d: '>', .. } => {
                let heuristic = heuristic(node.r, node.c + 1, &grid);
                add_node(Node {c: node.c + 1, d: '^', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {c: node.c + 1, d: '>', cost, heuristic, history: (0, node.history.1 + 1, 0, 0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {c: node.c + 1, d: 'v', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
            }
            Node {d: 'v', .. } => {
                let heuristic = heuristic(node.r + 1, node.c, &grid);
                add_node(Node {r: node.r + 1, d: '<', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {r: node.r + 1, d: 'v', cost, heuristic, history: (0, 0, node.history.2 + 1, 0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {r: node.r + 1, d: '>', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
            }
            Node {d: '<', .. } => {
                let heuristic = heuristic(node.r, node.c - 1, &grid);
                add_node(Node {c: node.c - 1, d: '^', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {c: node.c - 1, d: '<', cost, heuristic, history: (0, 0, 0, node.history.3 + 1), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {c: node.c - 1, d: 'v', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
            }
            Node {d: '^', .. } => {
                let heuristic = heuristic(node.r - 1, node.c, &grid);
                add_node(Node {r: node.r - 1, d: '<', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {r: node.r - 1, d: '^', cost, heuristic, history: (node.history.0 + 1, 0, 0, 0), path: path.clone(), ..node}, &mut heap, &seen);
                add_node(Node {r: node.r - 1, d: '>', cost, heuristic, history: (0,0,0,0), path: path.clone(), ..node}, &mut heap, &seen);
            }
            _ => panic!("Could not match the Node!")
        }
        seen.insert(node.clone());
        node = heap.pop().unwrap();
    }
    println!("Final: {:?} -> {:?}", node, path_cost(node.clone().path, &grid));
    (Some(path_cost(node.clone().path, &grid).to_string()), None)
}

mod tests {
    use crate::days::day17::execute;

    #[test]
    fn test_execute_with_specific_input() {
        let input: Vec<String> = vec![
            "2413432311323",
            "3215453535623",
            "3255245654254",
            "3446585845452",
            "4546657867536",
            "1438598798454",
            "4457876987766",
            "3637877979653",
            "4654967986887",
            "4564679986453",
            "1224686865563",
            "2546548887735",
            "4322674655533"
        ].into_iter().map(|s|s.to_string()).collect();
        execute(input);
    }
}