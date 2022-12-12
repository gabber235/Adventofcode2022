use rayon::prelude::*;
use std::fs;

fn main() {
    // Read the text from the file input.txt
    let text = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    println!("Part1: {}", process_part1(text.clone()));
    println!("Part2: {}", process_part2(text));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Node {
    Start {
        x: u32,
        y: u32,
    },
    End {
        x: u32,
        y: u32,
        parent: Option<(u32, u32)>,
    },
    Step {
        x: u32,
        y: u32,
        elevation: u8,
        cost: u32,
        heuristic: u32,
        parent: Option<(u32, u32)>,
    },
}

impl Node {
    fn x(&self) -> u32 {
        match self {
            Node::Start { x, .. } => *x,
            Node::End { x, .. } => *x,
            Node::Step { x, .. } => *x,
        }
    }

    fn y(&self) -> u32 {
        match self {
            Node::Start { y, .. } => *y,
            Node::End { y, .. } => *y,
            Node::Step { y, .. } => *y,
        }
    }

    fn cost(&self) -> u32 {
        match self {
            Node::Start { .. } => 0,
            Node::End { .. } => 0,
            Node::Step { cost, .. } => *cost,
        }
    }

    fn heuristic(&self) -> u32 {
        match self {
            Node::Start { .. } => 0,
            Node::End { .. } => 0,
            Node::Step { heuristic, .. } => *heuristic,
        }
    }

    fn parent(&self) -> Option<(u32, u32)> {
        match self {
            Node::Start { .. } => None,
            Node::End { parent, .. } => *parent,
            Node::Step { parent, .. } => *parent,
        }
    }

    fn elevation(&self) -> u8 {
        match self {
            Node::Start { .. } => 0,
            Node::End { .. } => 25,
            Node::Step { elevation, .. } => *elevation,
        }
    }

    fn score(&self) -> u32 {
        self.cost() + self.heuristic()
    }
}

fn parse_node(c: char, x: u32, y: u32) -> Node {
    match c {
        'S' => Node::Start { x, y },
        'E' => Node::End { x, y, parent: None },
        _ => Node::Step {
            x,
            y,
            elevation: c as u8 - b'a',
            cost: u32::MAX,
            heuristic: 0,
            parent: None,
        },
    }
}

fn parse_line(line: &str, y: u32) -> Vec<Node> {
    line.chars()
        .enumerate()
        .map(|(x, c)| parse_node(c, x as u32, y))
        .collect()
}

fn parse_input(text: String) -> Vec<Vec<Node>> {
    text.lines()
        .enumerate()
        .map(|(y, line)| parse_line(line, y as u32))
        .collect()
}

fn find_start(nodes: &[Vec<Node>]) -> Option<Node> {
    nodes
        .iter()
        .flat_map(|line| line.iter())
        .find(|node| matches!(node, Node::Start { .. }))
        .cloned()
}

fn find_end(nodes: &[Vec<Node>]) -> Option<Node> {
    nodes
        .iter()
        .flat_map(|line| line.iter())
        .find(|node| matches!(node, Node::End { .. }))
        .cloned()
}

fn calculate_heuristic(node: &mut Node, end: &Node) {
    let dx = (node.x() as i32 - end.x() as i32).abs();
    let dy = (node.y() as i32 - end.y() as i32).abs();
    let heuristic = (dx + dy) as u32;
    if let Node::Step { heuristic: h, .. } = node {
        *h = heuristic
    }
}

fn update_heuristic(nodes: &mut Vec<Vec<Node>>) {
    let end = find_end(nodes).unwrap();
    for row in nodes {
        for node in row {
            if let Node::Step { .. } = node {
                calculate_heuristic(node, &end)
            }
        }
    }
}

fn get_neighbors(nodes: &[Vec<Node>], node: &Node) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::new();
    let x = node.x() as usize;
    let y = node.y() as usize;
    let elevation = node.elevation();
    if x > 0 {
        let neighbor = &nodes[y][x - 1];
        if neighbor.elevation() <= elevation + 1 {
            neighbors.push((neighbor.x(), neighbor.y()))
        }
    }
    if x < nodes[0].len() - 1 {
        let neighbor = &nodes[y][x + 1];
        if neighbor.elevation() <= elevation + 1 {
            neighbors.push((neighbor.x(), neighbor.y()))
        }
    }
    if y > 0 {
        let neighbor = &nodes[y - 1][x];
        if neighbor.elevation() <= elevation + 1 {
            neighbors.push((neighbor.x(), neighbor.y()))
        }
    }
    if y < nodes.len() - 1 {
        let neighbor = &nodes[y + 1][x];
        if neighbor.elevation() <= elevation + 1 {
            neighbors.push((neighbor.x(), neighbor.y()))
        }
    }
    neighbors
}

#[test]
fn test_get_neighbors() {
    let input = "aby\nacx";
    let nodes = parse_input(input.to_string());
    let node = &nodes[0][1];
    let neighbors = get_neighbors(&nodes, node);
    assert_eq!(neighbors.len(), 2);
}

fn find_node_mut(nodes: &mut [Vec<Node>], x: u32, y: u32) -> Option<&mut Node> {
    nodes
        .iter_mut()
        .flat_map(|line| line.iter_mut())
        .find(|node| node.x() == x && node.y() == y)
}

fn find_node(nodes: &[Vec<Node>], x: u32, y: u32) -> Option<&Node> {
    nodes
        .iter()
        .flat_map(|line| line.iter())
        .find(|node| node.x() == x && node.y() == y)
}

fn find_shortest_path(nodes: &mut [Vec<Node>]) -> Option<u32> {
    let start = find_start(nodes).unwrap();
    let mut open = vec![(start.x(), start.y())];
    let mut closed = vec![];
    while let Some(cord) = open.pop() {
        let node = find_node(nodes, cord.0, cord.1).unwrap();
        if matches!(node, Node::End { .. }) {
            // Return the amount of steps
            let mut steps = 0;
            let mut current = node;
            while let Some(parent) = current.parent() {
                steps += 1;
                current = find_node(nodes, parent.0, parent.1).unwrap();
            }
            return Some(steps);
        }
        closed.push(cord);
        let cost = node.cost() + 1;
        let neighbors = get_neighbors(nodes, node);
        for neighbor in neighbors {
            if closed.contains(&neighbor) {
                continue;
            }

            let neighbor_node = find_node(nodes, neighbor.0, neighbor.1).unwrap();

            if !open.contains(&neighbor) || cost < neighbor_node.cost() {
                let mut neighbor = find_node_mut(nodes, neighbor.0, neighbor.1).unwrap();
                if let Node::Step {
                    cost: c, parent: p, ..
                } = &mut neighbor
                {
                    *c = cost;
                    *p = Some(cord);
                }
                if let Node::End { parent: p, .. } = &mut neighbor {
                    *p = Some(cord);
                }
                open.push((neighbor.x(), neighbor.y()));
            }
        }
        open.sort_by(|a, b| {
            let a = find_node(nodes, a.0, a.1).unwrap();
            let b = find_node(nodes, b.0, b.1).unwrap();
            let a_cost = a.score();
            let b_cost = b.score();
            // Sort in reverse order since we pop from the end
            b_cost.cmp(&a_cost)
        });
    }
    None
}

fn process_part1(text: String) -> u32 {
    let mut grid = parse_input(text);

    // Update the heuristic for each node
    update_heuristic(&mut grid);

    // Find the shortest path
    find_shortest_path(&mut grid).unwrap()
}

fn replace_start_point(mut grid: Vec<Vec<Node>>, start: (u32, u32)) -> Vec<Vec<Node>> {
    grid[start.1 as usize][start.0 as usize] = Node::Start {
        x: start.0,
        y: start.1,
    };
    grid
}

fn process_part2(text: String) -> u32 {
    let mut grid = parse_input(text);

    // Remove the start node
    let start = find_start(&grid).unwrap();
    grid[start.y() as usize][start.x() as usize] = Node::Step {
        x: start.x(),
        y: start.y(),
        cost: u32::MAX,
        elevation: 0,
        heuristic: 0,
        parent: None,
    };

    // Update the heuristic for each node
    update_heuristic(&mut grid);

    // Find all positions with elevation a
    let mut positions = vec![];
    for row in &grid {
        for node in row {
            if node.elevation() == 0 {
                positions.push((node.x(), node.y()));
            }
        }
    }

    let mut graphs = positions
        .iter()
        .map(|pos| replace_start_point(grid.clone(), *pos))
        .collect::<Vec<_>>();

    // Find the shortest path for each position as start position and return the lowest
    graphs
        .par_iter_mut()
        .filter_map(|graph| find_shortest_path(graph))
        .min()
        .unwrap()
}
