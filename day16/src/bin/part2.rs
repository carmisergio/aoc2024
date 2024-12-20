use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    io::{stdin, BufReader},
    process,
};

use day16::{parse_input, Cell, Direction, DisplayChar, Matrix2D, Position};

fn main() {
    // Read input
    let (matrix, start, end) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Solve
    let res = count_seats(&matrix, start, end);

    // Print result
    println!("Result: {}", res);
}

fn count_seats(matrix: &Matrix2D<Cell>, start: Position, end: Position) -> usize {
    let paths = find_best_paths(matrix, start, end);
    print_matrix_seats(matrix, &paths);
    paths.len()
}

fn find_best_paths(matrix: &Matrix2D<Cell>, start: Position, end: Position) -> HashSet<Position> {
    // Dijkstra state
    let mut tmp_dist: BTreeSet<TempDistanceNode> = BTreeSet::new();
    let mut fin_dist: HashMap<(Position, Direction), usize> = HashMap::new();
    let mut path: HashSet<Position> = HashSet::new();
    let mut best_end: Option<usize> = None;

    // Insert starting node
    tmp_dist.insert(TempDistanceNode {
        pos: start,
        dir: Direction::Right,
        dist: 0,
        path: HashSet::from([start]),
    });

    while let Some(node) = tmp_dist.pop_first() {
        // Check if we've reached the end
        if node.pos == end {
            if let Some(best_end) = best_end {
                if node.dist > best_end {
                    continue;
                }
            }

            // Add this node's path to the global path
            path.extend(node.path.iter());

            // Keep track of the best distance to the end
            best_end = Some(node.dist);
        }

        // The node has already been fixed at a better distance
        if let Some(prev_dist) = fin_dist.get(&(node.pos, node.dir)) {
            if node.dist > *prev_dist {
                continue;
            }
        }

        // Fix this node's distance
        fin_dist.insert((node.pos, node.dir), node.dist);

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for dir in directions {
            // Moved
            let pos = node.pos.moved(dir);

            // Check if we can go there
            if let Some(cell) = matrix.get(pos) {
                // There's a wall
                if cell != Cell::Empty {
                    continue;
                }
            } else {
                // Outside of the map
                continue;
            }

            // Calculate new distance
            let mut dist = node.dist + 1;
            if dir != node.dir {
                if dir.is_orthogonal(&node.dir) {
                    dist += 1000; // 90 deg turn
                } else {
                    // dist += 2000;
                    continue;
                }
            }

            let mut new_path = node.path.clone();
            new_path.insert(pos);

            // Add new tmporary distance node
            tmp_dist.insert(TempDistanceNode {
                pos,
                dist,
                dir,
                path: new_path,
            });
        }

        // dbg!(&tmp_dist);
        // dbg!(&fin_dist);
    }

    path
}

#[derive(Debug, Clone)]
struct TempDistanceNode {
    pub pos: Position,
    pub dist: usize,
    pub dir: Direction,
    pub path: HashSet<Position>,
}

impl PartialEq for TempDistanceNode {
    fn eq(&self, other: &TempDistanceNode) -> bool {
        self.pos == other.pos && self.dir == other.dir && self.path == other.path
    }
}

impl Eq for TempDistanceNode {}

impl PartialOrd for TempDistanceNode {
    fn partial_cmp(&self, other: &TempDistanceNode) -> Option<Ordering> {
        if self.pos == other.pos && self.dir == other.dir && self.path == other.path {
            return Some(Ordering::Equal);
        }

        if self.dist > other.dist {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for TempDistanceNode {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.pos == other.pos && self.dir == other.dir && self.path == other.path {
            return Ordering::Equal;
        }
        if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

fn print_matrix_seats(matrix: &Matrix2D<Cell>, paths: &HashSet<Position>) {
    for y in 0..matrix.height() {
        for x in 0..matrix.width() {
            let pos = Position::new_usize(x, y);
            let cell = matrix.get(pos).unwrap();
            if paths.contains(&pos) {
                print!("O");
            } else {
                print!("{}", cell.display_char());
            }
        }
        println!()
    }
}
