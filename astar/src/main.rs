use rand::Rng;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt;
use std::usize;
use priority_queue::PriorityQueue;

#[macro_use]
extern crate lazy_static;

use std::sync::Arc;


lazy_static! {
    static ref GOAL: Arc<Vec<Vec<u8>>> = Arc::new(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 0],
    ]);
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Puzzle {
    board: Vec<Vec<u8>>,
}

struct GoalPositions {
    positions: Vec<(usize, usize)>,
}

impl GoalPositions {
    fn new(goal: &Vec<Vec<u8>>) -> Self {
        let size = goal.len();
        let mut positions = vec![(0, 0); size * size];
        for (i, row) in goal.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                positions[value as usize] = (i, j);
            }
        }
        Self { positions }
    }

    fn get(&self, value: u8) -> (usize, usize) {
        self.positions[value as usize]
    }
}

impl Puzzle {
    fn new(initial: Vec<Vec<u8>>) -> Self {
        Puzzle {
            board: initial.clone(),
            
        }
    }

    fn is_goal(&self) -> bool {
        self.board == *GOAL.clone() 
    }

    fn display(&self) {
        for row in self.board.iter() {
            for &val in row.iter() {
                print!("{} ", val);
            }
            println!();
        }
        println!();
    }

    fn find_zero(&self) -> (usize, usize) {
        for (i, row) in self.board.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                if val == 0 {
                    return (i, j);
                }
            }
        }
        panic!("No zero found in the puzzle");
    }

    fn swap(&mut self, row1: usize, col1: usize, row2: usize, col2: usize) {
        let temp = self.board[row1][col1];
        self.board[row1][col1] = self.board[row2][col2];
        self.board[row2][col2] = temp;
    }
}

impl fmt::Debug for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.board.iter() {
            write!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Node {
    puzzle: Puzzle,
    g_cost: usize,
    h_cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.g_cost + self.h_cost)
            .cmp(&(other.g_cost + other.h_cost))
            .reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn astar(initial_state: Puzzle) -> Option<Vec<Puzzle>> {
    let mut open_set = BinaryHeap::new();
    let mut g_costs: HashMap<Puzzle, usize> = HashMap::new();
    let mut came_from = HashMap::new();
    let goal_positions = GoalPositions::new(&GOAL.clone());
    let initial_node = Node {
        puzzle: initial_state.clone(),
        g_cost: 0,
        h_cost: heuristic(&initial_state,&goal_positions),
    };

    open_set.push(initial_node.clone());
    g_costs.insert(initial_state.clone(), 0);

    while let Some(current_node) = open_set.pop() {
        if current_node.puzzle.is_goal() {
            return Some(reconstruct_path(came_from, current_node));
        }

        for neighbor in get_neighbors(&current_node.puzzle) {
            let tentative_g_cost = current_node.g_cost + 1;

            if !g_costs.contains_key(&neighbor)
                || tentative_g_cost < *g_costs.get(&neighbor).unwrap()
            {
                g_costs.insert(neighbor.clone(), tentative_g_cost);
                let h_cost = heuristic(&neighbor, &goal_positions);
                open_set.push(Node {
                    puzzle: neighbor.clone(),
                    g_cost: tentative_g_cost,
                    h_cost,
                });

                came_from.insert(neighbor, current_node.puzzle.clone());
            }
        }
    }

    None
}

fn heuristic(puzzle: &Puzzle, goal_positions: &GoalPositions) -> usize {
    let mut total_distance = 0;

    for (i, row) in puzzle.board.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value != 0 {
                let (goal_i, goal_j) = goal_positions.get(value);
                total_distance += (i as i32 - goal_i as i32).abs() as usize;
                total_distance += (j as i32 - goal_j as i32).abs() as usize;
            }
        }
    }

    total_distance
}

fn find_position(grid: &Vec<Vec<u8>>, val: u8) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == val) {
            return (i, j);
        }
    }
    unreachable!()
}
fn get_neighbors(puzzle: &Puzzle) -> Vec<Puzzle> {
    let mut neighbors = Vec::new();
    let (zero_row, zero_col) = puzzle.find_zero();

    for &(dr, dc) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let new_row = (zero_row as isize + dr) as usize;
        let new_col = (zero_col as isize + dc) as usize;

        if new_row < puzzle.board.len() && new_col < puzzle.board[0].len() {
            let mut new_board = puzzle.board.clone();
            new_board[zero_row][zero_col] = new_board[new_row][new_col];
            new_board[new_row][new_col] = 0;
            neighbors.push(Puzzle {
                board: new_board,
            });
        }
    }
    neighbors
}

fn reconstruct_path(came_from: HashMap<Puzzle, Puzzle>, mut current: Node) -> Vec<Puzzle> {
    let mut path = vec![current.puzzle.clone()];
    while let Some(prev) = came_from.get(&current.puzzle) {
        path.push(prev.clone());
        current = Node {
            puzzle: prev.clone(),
            g_cost: 0,
            h_cost: 0,
        };
    }
    path.reverse();
    path
}

fn create_puzzle(n: usize) -> Vec<Vec<u8>> {
    let mut puzzle = vec![vec![0; n]; n];
    let mut value = 1;

    for i in 0..n {
        for j in 0..n {
            puzzle[i][j] = value;
            value += 1;
        }
    }
    puzzle[n - 1][n - 1] = 0;

    puzzle
}

fn mix_puzzle(init: Vec<Vec<u8>>, rd: i32) -> Puzzle {
    let mut  puzzle = Puzzle::new(init);
    for _ in 0..rd{
        let move_possible = get_neighbors(&puzzle);
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..move_possible.len());
        puzzle = move_possible[index].clone();
    }
    puzzle
}

fn main() {
 //   let initial_state: Vec<Vec<u8>> = create_puzzle(3);


   /*  let tableau = vec![
        vec![10, 13, 0, 9],
        vec![14, 2, 6, 1],
        vec![12, 3, 11, 7],
        vec![4, 15, 5, 8],
    ];*/

    let initial_state = vec![
        vec![6, 4, 7],
        vec![8, 5, 0],
        vec![3, 2, 1],
    ];
    //let puzzle = mix_puzzle(initial_state,25);
    let puzzle = Puzzle::new(initial_state);
  // let puzzle = mix_puzzle(initial_state,100 );
    let start_time = std::time::Instant::now();
    if let Some(solution) = astar(puzzle) {
        let elapsed = start_time.elapsed();

        for (i, step) in solution.iter().enumerate() {
            println!("Step {}: ", i + 1);
            step.display();
        }
        println!(
            "Solution found in {} steps and {},{} s.",
            solution.len() - 1,
            elapsed.as_secs(),
            elapsed.subsec_millis()
        );
    } else {
        println!("No solution found.");
    }
}
