use std::{
    cmp::Ordering,
    collections::BTreeSet,
    io::{stdin, BufReader},
    process,
};

use day16::{parse_input, Cell, Direction, Matrix2D, Position};

fn main() {
    // Read input
    let (matrix, start_pos, end_pos) = parse_input(BufReader::new(stdin())).unwrap_or_else(|e| {
        eprintln!("Error reading input: {}", e);
        process::exit(1);
    });

    // Solve
    let res = solve(&matrix, start_pos, end_pos).unwrap_or_else(|| {
        eprintln!("Destination unreachable");
        process::exit(1);
    });

    // Print result
    println!("Result: {}", res);
}

fn solve(matrix: &Matrix2D<Cell>, start_pos: Position, end_pos: Position) -> Option<usize> {
    let mut final_distances = Matrix2D::new(matrix.width(), matrix.height(), None);

    // Calculate dijkstra
    dijkstra_from(
        matrix,
        &mut final_distances,
        start_pos,
        Some(Direction::Right),
    );

    // Get distance to end node
    let dist = final_distances
        .get(end_pos)
        .expect("end position outside map")?;

    return Some(dist);
}

#[derive(Debug, Clone, Copy)]
struct TempDistanceNode {
    pub pos: Position,
    pub dist: usize,
    pub dir: Option<Direction>,
}

impl PartialEq for TempDistanceNode {
    fn eq(&self, other: &TempDistanceNode) -> bool {
        self.pos == other.pos && self.dir == other.dir
    }
}

impl Eq for TempDistanceNode {}

impl PartialOrd for TempDistanceNode {
    fn partial_cmp(&self, other: &TempDistanceNode) -> Option<Ordering> {
        if self.pos == other.pos && self.dir == other.dir {
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
        if self.pos == other.pos && self.dir == other.dir {
            return Ordering::Equal;
        }
        if self.dist > other.dist {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

fn dijkstra(
    matrix: &Matrix2D<Cell>,
    final_distances: &mut Matrix2D<Option<usize>>,
    tmp_distances: &mut BTreeSet<TempDistanceNode>,
) {
    loop {
        // Extract current node
        let node = *match tmp_distances.first() {
            None => return,
            Some(node) => node,
        };

        // dbg!(&tmp_distances);
        // dbg!(&node);

        tmp_distances.remove(&node);

        // Check if we can go there
        if let Some(cell) = matrix.get(node.pos) {
            // There's a wall
            if cell != Cell::Empty {
                continue;
            }

            // The distance has already been fixed
            if let Some(Some(_)) = final_distances.get(node.pos) {
                continue;
            }
        } else {
            continue;
        }

        // Fix this node's distance
        final_distances.set(node.pos, Some(node.dist));

        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        for dir in directions {
            // Moved
            let pos = node.pos.moved(dir);

            // Calculate new distance
            let mut dist = node.dist + 1;
            if let Some(prev_dir) = node.dir {
                if prev_dir.is_orthogonal(&dir) {
                    dist += 1000;
                }
            }

            // Add new tmporary distance node
            tmp_distances.insert(TempDistanceNode {
                pos,
                dist,
                dir: Some(dir),
            });
        }
    }
}

fn dijkstra_from(
    matrix: &Matrix2D<Cell>,
    final_distances: &mut Matrix2D<Option<usize>>,
    start: Position,
    dir: Option<Direction>,
) {
    let mut tmp_distances = BTreeSet::<TempDistanceNode>::new();

    // Start node is at distance 0 from itself
    tmp_distances.insert(TempDistanceNode {
        pos: start,
        dist: 0,
        dir,
    });

    // Compute distances
    dijkstra(matrix, final_distances, &mut tmp_distances);
}
