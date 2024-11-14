
fn is_solution(configuration: &Vec<char>) -> bool {

    let middle_position = configuration.len() / 2;
    if configuration[middle_position] != '_' {
        return false;
    }
    let expected: Vec<char> = vec!['R'; middle_position].into_iter().chain(vec!['_']).chain(vec!['L'; middle_position]).collect();
    configuration == &expected
}

fn possible_moves(configuration: &Vec<char>) -> Vec<(usize, usize)> {
    let mut moves = vec![];
    for i in 0..configuration.len() {
        match configuration[i] {
            'L' => {       
                if i < configuration.len() - 1 && configuration[i+1] == '_' {
                    moves.push((i, i+1));
                } else if i < configuration.len() - 2 && configuration[i+2] == '_' {
                    moves.push((i, i+2));
                }
            },
            'R' => {
                if i > 0 && configuration[i-1] == '_' {
                    moves.push((i, i-1));
                } else if i > 1 && configuration[i-2] == '_' {
                    moves.push((i, i-2));
                }
            },
            _ => {}
        }
    }
    moves
}

fn dfs(initial_configuration: Vec<char>) -> bool {
    let mut stack = vec![(initial_configuration, vec![])];
   
    while let Some((configuration, path)) = stack.pop() {
        let configuration_str: String = configuration.iter().collect();
        if is_solution(&configuration) {
            println!("{}", path.into_iter().chain(vec![configuration_str]).collect::<Vec<_>>().join("\n"));
            
            return true;
        }
        for (i, j) in possible_moves(&configuration) {
            let mut new_configuration = configuration.clone();
            new_configuration.swap(i, j);
            let new_path = path.clone().into_iter().chain(vec![configuration_str.clone()]).collect();
            stack.push((new_configuration, new_path));
        }
    }
    false
}


pub fn solve_dfs(n:usize) {
    let initial_configuration: Vec<char> = vec!['L'; n].into_iter().chain(vec!['_']).chain(vec!['R'; n]).collect();
    dfs(initial_configuration);
}

