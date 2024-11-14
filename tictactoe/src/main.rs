use std::io::{self, Write};
use std::time::Instant;

type Board = Vec<Vec<char>>;

fn print_board(board: &Board) {
    println!("   1   2   3");
    println!(" +---+---+---+");
    for (i, row) in board.iter().enumerate() {
        print!("{}| ", i + 1);
        for &cell in row {
            print!("{} | ", cell);
        }
        println!("\n +---+---+---+");
    }
}

fn is_winner(board: &Board, player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    false
}

fn is_draw(board: &Board) -> bool {
    for row in board {
        for &cell in row {
            if cell == ' ' {
                return false;
            }
        }
    }
    true
}

fn play_move(board: &mut Board, player: char, row: usize, col: usize) {
    board[row][col] = player;
}

fn possible_moves(board: &Board) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == ' ' {
                moves.push((i, j));
            }
        }
    }
    moves
}

fn current_player(board: &Board) -> char {
    let mut x_count = 0;
    let mut o_count = 0;
    for row in board {
        for &cell in row {
            if cell == 'X' {
                x_count += 1;
            } else if cell == 'O' {
                o_count += 1;
            }
        }
    }
    if x_count <= o_count { 'X' } else { 'O' }
}

fn minimax(board: &mut Board, player: char) -> i32 {
    if is_winner(board, 'X') {
        return 1;
    }
    if is_winner(board, 'O') {
        return -1;
    }
    if is_draw(board) {
        return 0;
    }

    let mut best_score = if player == 'X' { -1 } else { 1 };
    for (i, j) in possible_moves(board) {
        play_move(board, player, i, j);
        let score = minimax(board, if player == 'X' { 'O' } else { 'X' });
        play_move(board, ' ', i, j);
        if player == 'X' {
            best_score = best_score.max(score);
        } else {
            best_score = best_score.min(score);
        }
    }
    best_score
}

fn best_move_minimax(board: &mut Board, player: char) -> (usize, usize) {
    let mut best_score = if player == 'X' { -1 } else { 1 };
    let mut best_move = (0, 0);
    for (i, j) in possible_moves(board) {
        play_move(board, player, i, j);
        let score = minimax(board, if player == 'X' { 'O' } else { 'X' });
        play_move(board, ' ', i, j);
        if player == 'X' {
            if score > best_score {
                best_score = score;
                best_move = (i, j);
            }
        } else {
            if score < best_score {
                best_score = score;
                best_move = (i, j);
            }
        }
    }
    best_move
}


fn alphabeta(board: &mut Board, player: char, mut alpha: i32, mut beta: i32) -> i32 {
    if is_winner(board, 'X') {
        return  1;
    }
    if is_winner(board, 'O') {
        return -1;
    }
    if is_draw(board) {
        return 0;
    }

    let mut best_score = if player == 'X' { -1 } else { 1 };
    for (i, j) in possible_moves(board) {
        play_move(board, player, i, j);
        let score = alphabeta(board, if player == 'X' { 'O' } else { 'X' }, alpha, beta);
        play_move(board, ' ', i, j);
        if player == 'X' {
            best_score = best_score.max(score);
            alpha = alpha.max(score);
        } else {
            best_score = best_score.min(score);
            beta = beta.min(score);
        }
        if beta <= alpha {
            break;
        }
    }
    best_score
}

fn best_move_alphabeta(board: &mut Board, player: char) -> (usize, usize) {
    let mut best_score = if player == 'X' { -1 } else { 1 };
    let mut best_move: (usize, usize) = (0, 0);
    let mut alpha = -1;
    let mut beta = 1;
    for (i, j) in possible_moves(board) {
        play_move(board, player, i, j);
        let score = alphabeta(board, if player == 'X' { 'O' } else { 'X' }, alpha, beta);
        play_move(board, ' ', i, j);
        if player == 'X' {
            if score > best_score {
                best_score = score;
                best_move = (i, j);
            }
            alpha = alpha.max(score);
        } else {
            if score < best_score {
                best_score = score;
                best_move = (i, j);
            }
            beta = beta.min(score);
        }
        if beta <= alpha {
            break;
        }
    }
    best_move
}

fn ask_user_for_algorithm() -> String {
    let mut algorithm = String::new();
    println!("Which algorithm do you want to use? (minmax/alpha-beta)");
    io::stdin().read_line(&mut algorithm).expect("Failed to read line");
    algorithm.trim().to_string()
}

fn ask_user_for_first_player() -> char {
    let mut player = String::new();
    println!("Who should play first? (me/computer)");
    io::stdin().read_line(&mut player).expect("Failed to read line");
    match player.trim() {
        "me" => 'X',
        "computer" => 'O',
        _ => {
            println!("Invalid choice, try again.");
            ask_user_for_first_player()
        }
    }
}


fn main() {
    let mut board = vec![vec![' '; 3]; 3];
    let mut player = ask_user_for_first_player();
    let mut game_over = false;
    let mut winner = ' ';
    

    let algorithm = ask_user_for_algorithm();

    while !game_over {
        print_board(&board);
        let (row, col) = if player == 'X' {
            print!("Player X, enter your move (row col): ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let row = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let col = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            (row, col)
        } else {
            let start = Instant::now();
            let (row, col) = match algorithm.as_str() {
                "minmax" => best_move_minimax(&mut board, player),
                "alpha-beta" => best_move_alphabeta(&mut board, player),
                _ => {
                    println!("Invalid choice, try again.");
                    continue;
                }
            };
            let duration = start.elapsed();
            println!("Player O, thinking time: {:?}", duration);
            (row, col)
        };
        if board[row][col] == ' ' {
            play_move(&mut board, player, row, col);
            if is_winner(&board, player) {
                game_over = true;
                winner = player;
                print_board(&board);
                println!("Player {} wins!", winner);
            } else if is_draw(&board) {
                game_over = true;
                print_board(&board);
                println!("It's a draw!");
            }
            player = if player == 'X' { 'O' } else { 'X' };
        } else {
            println!("Invalid move, try again.");
        }
    }
}