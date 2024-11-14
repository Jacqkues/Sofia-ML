use std::collections::HashSet;

fn is_goal_state(configuration: &Vec<char>) -> bool {

    let middle_position = configuration.len() / 2;
    if configuration[middle_position] != '_' {
        return false;
    }
    let expected: Vec<char> = vec!['R'; middle_position].into_iter().chain(vec!['_']).chain(vec!['L'; middle_position]).collect();
    configuration == &expected
}

fn moves(configuration: &Vec<char>) -> Vec<Vec<char>> {
    let mut moves = Vec::new();
    for i in 0..configuration.len() {
        match configuration[i] {
            'L' => {       
                if i < configuration.len() - 1 && configuration[i+1] == '_' {
                    let mut new_configuration = configuration.clone();
                    new_configuration.swap(i, i+1);
                    moves.push(new_configuration);
                } else if i < configuration.len() - 2 && configuration[i+2] == '_' {
                    let mut new_configuration = configuration.clone();
                    new_configuration.swap(i, i+2);
                    moves.push(new_configuration);
                }
            },
            'R' => {
                if i > 0 && configuration[i-1] == '_' {
                    let mut new_configuration = configuration.clone();
                    new_configuration.swap(i, i-1);
                    moves.push(new_configuration);
                } else if i > 1 && configuration[i-2] == '_' {
                    let mut new_configuration = configuration.clone();
                    new_configuration.swap(i, i-2);
                    moves.push(new_configuration);
                }
            },
            _ => {}
        }
    }
    moves
}


fn dfs_rec(board: &Vec<char>) -> bool {
    if is_goal_state(&board) {
        return true;
    }
    for new_board in moves(&board) {
        if dfs_rec(&new_board) {
            println!("{}", new_board.iter().rev().collect::<String>());
            return true;
        }
    }
    false
}


pub fn solve_dfs_rec(n:usize){
    
    let board: Vec<char> = vec!['L'; n].into_iter().chain(vec!['_']).chain(vec!['R'; n]).collect();
    let result = dfs_rec(&board);
    
}

fn dfs_iter(board: &mut Vec<char>, visited: &mut HashSet<Vec<char>>) -> bool {
    let mut stack = vec![board.clone()];

    while let Some(mut current_board) = stack.pop() {
        if is_goal_state(&current_board) {
            return true;
        }

        for i in 0..current_board.len() {
            match current_board[i] {
                'L' => {
                    if i < current_board.len() - 1 && current_board[i+1] == '_' {
                        current_board.swap(i, i+1);
                        if visited.insert(current_board.clone()) {
                            stack.push(current_board.clone());
                        }
                        current_board.swap(i, i+1);
                    }
                    if i < current_board.len() - 2 && current_board[i+2] == '_' {
                        current_board.swap(i, i+2);
                        if visited.insert(current_board.clone()) {
                            stack.push(current_board.clone());
                        }
                        current_board.swap(i, i+2);
                    }
                },
                'R' => {
                    if i > 0 && current_board[i-1] == '_' {
                        current_board.swap(i, i-1);
                        if visited.insert(current_board.clone()) {
                            stack.push(current_board.clone());
                        }
                        current_board.swap(i, i-1);
                    }
                    if i > 1 && current_board[i-2] == '_' {
                        current_board.swap(i, i-2);
                        if visited.insert(current_board.clone()) {
                            stack.push(current_board.clone());
                        }
                        current_board.swap(i, i-2);
                    }
                },
                _ => {}
            }
        }
    }

    false
}

pub fn solve_dfs_iter(n: usize) {
    let mut board: Vec<char> = vec!['L'; n].into_iter().chain(vec!['_']).chain(vec!['R'; n]).collect();
    let mut visited = HashSet::new();
    let result = dfs_iter(&mut board, &mut visited);
}