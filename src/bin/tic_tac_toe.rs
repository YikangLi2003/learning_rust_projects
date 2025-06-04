use rand::Rng;
use std::io;


const SPACE: char = ' ';
const NOUGHT: char = 'O';
const CROSS: char = 'X';


fn main() {
    let mut board = [[SPACE; 3]; 3];

    println!("Welcome to Tic Tac Toe.");
    render_board(&board);
    
    loop {
        manual_player_move(&mut board, CROSS);
        render_board(&board);
        
        let winner = check_winner(&board);
        if winner != SPACE {
            println!("Player '{}' won the game!", winner);
            break;
        }

        println!("Computer agent just placed a piece.");
        random_agent_move(&mut board, NOUGHT);
        render_board(&board);

        let winner = check_winner(&board);
        if winner != SPACE {
            println!("Player '{}' won the game!", winner);
            break;
        }
    }

    
}


fn render_board(board: &[[char; 3]; 3]) {
    println!("| {} | {} | {} |", board[0][0], board[0][1], board[0][2]);
    println!("| {} | {} | {} |", board[1][0], board[1][1], board[1][2]);
    println!("| {} | {} | {} |", board[2][0], board[2][1], board[2][2]);
    println!("");
}


fn check_winner(board: &[[char; 3]; 3]) -> char {
    let winner_patterns: [[[usize; 2]; 3]; 8] = [
        [[0, 0], [0, 1], [0, 2]],
        [[1, 0], [1, 1], [1, 2]],
        [[2, 0], [2, 1], [2, 2]],
        [[0, 0], [1, 0], [2, 0]],
        [[0, 1], [1, 1], [2, 1]],
        [[0, 2], [1, 2], [2, 2]],
        [[0, 0], [1, 1], [2, 2]],
        [[0, 2], [1, 1], [2, 0]],
    ];

    for pattern in winner_patterns {
        let piece_0 = board[pattern[0][0]][pattern[0][1]];
        let piece_1 = board[pattern[1][0]][pattern[1][1]];
        let piece_2 = board[pattern[2][0]][pattern[2][1]];

        if piece_0 == piece_1 && piece_1 == piece_2 {
            if piece_0 == CROSS {
                return CROSS;
            } else if piece_0 == NOUGHT {
                return NOUGHT;
            }
        }
    }

    SPACE
}


fn random_agent_move(board: &mut [[char; 3]; 3], agent_piece: char) {
    let mut rng = rand::thread_rng();

    loop {
        let row: usize = rng.gen_range(0..3);
        let col: usize = rng.gen_range(0..3);

        if board[row][col] == SPACE {
            board[row][col] = agent_piece;
            break;
        }
    }
}


fn manual_player_move(board: &mut [[char; 3]; 3], player_piece: char) {
    loop {
        println!("Input two integers separated by space as coordinate of your piece (e.g. 1 2): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid input. Make two integers are separated by a space.");
            continue;
        }

        let row_result = parts[0].parse::<usize>();
        let col_result = parts[1].parse::<usize>();

        if let (Ok(row), Ok(col)) = (row_result, col_result) {
            if row >= 3 || col >= 3 {
                println!("Invalid coordiante. Make sure both row and col are between 0 and 2.");
                continue;
            }

            if board[row][col] != SPACE {
                println!("Invalid coordiante. Make sure there is no piece on the coordiante.");
                continue;
            }

            board[row][col] = player_piece;
            break;
        } else {
            println!("Invalid input. Make sure the two values are integers.");
            continue;
        }
    };
}