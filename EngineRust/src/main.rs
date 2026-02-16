use shakmaty::uci::UciMove;
use shakmaty::{Board, Chess, Color, Move, Piece, Position, Role, Square};
use std::cmp::{max, min};
use std::io::{self, Write}; // Do wczytywania inputu
use std::str::FromStr; // Do parsowania napisów // Standard UCI (np. "e2e4")

// Tablice Piece-Square Tables (PST)
// Wartości z perspektywy BIAŁYCH. Dla czarnych będziemy "odwracać" planszę.

#[rustfmt::skip]
const PAWN_TABLE: [i32; 64] = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
const KNIGHT_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

#[rustfmt::skip]
const BISHOP_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20
];

#[rustfmt::skip]
const ROOK_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

// Król w środkowej grze (chce się chować w rogu)
#[rustfmt::skip]
const KING_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

fn get_piece_value(role: Role) -> i32 {
    match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 20000,
    }
}

fn evaluate_board(game: &Chess) -> i32 {
    let board = game.board();
    let mut score = 0;

    for square in board.occupied() {
        let piece = board.piece_at(square).unwrap();
        let value = get_piece_value(piece.role);

        let pst_value = match piece.role {
            Role::Pawn => {
                PAWN_TABLE[usize::from(if piece.color == Color::White {
                    square
                } else {
                    square.flip_vertical()
                })]
            }
            Role::Knight => {
                KNIGHT_TABLE[usize::from(if piece.color == Color::White {
                    square
                } else {
                    square.flip_vertical()
                })]
            }
            Role::Bishop => {
                BISHOP_TABLE[usize::from(if piece.color == Color::White {
                    square
                } else {
                    square.flip_vertical()
                })]
            }
            Role::Rook => {
                ROOK_TABLE[usize::from(if piece.color == Color::White {
                    square
                } else {
                    square.flip_vertical()
                })]
            }
            Role::King => {
                KING_TABLE[usize::from(if piece.color == Color::White {
                    square
                } else {
                    square.flip_vertical()
                })]
            }
            Role::Queen => 0, // Hetman zazwyczaj nie ma tabeli (jest wszędzie dobry)
        };

        if piece.color == Color::White {
            score += value + pst_value;
        } else {
            score -= value + pst_value;
        }
    }
    score
}

fn print_board(game: &Chess) {
    let board = game.board();

    println!("  +-----------------+");
    for rank in (0..8).rev() {
        print!("{} | ", rank + 1); // Numer rzędu po lewej

        for file in 0..8 {
            let square_index = (rank as u32) * 8 + (file as u32);
            let square = Square::new(square_index);

            let char_to_print = match board.piece_at(square) {
                Some(piece) => {
                    let c = piece.role.char();
                    if piece.color == Color::White {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                }
                None => '.', // Puste pole
            };
            print!("{} ", char_to_print);
        }
        println!("|");
    }
    println!("  +-----------------+");
    println!("    a b c d e f g h");
}

fn minimax(game: &Chess, depth: u8, mut alpha: i32, mut beta: i32, maximizing_player: bool) -> i32 {
    if depth == 0 || game.is_game_over() {
        return evaluate_board(game);
    }

    let legal_moves = game.legal_moves();

    if maximizing_player {
        let mut max_eval = -100_000;

        for m in legal_moves {
            let mut new_game = game.clone();

            new_game.play_unchecked(&m);

            let eval = minimax(&new_game, depth - 1, alpha, beta, false);
            max_eval = max(max_eval, eval);
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        return max_eval;
    } else {
        let mut min_eval = 100_000; // Plus nieskończoność

        for m in legal_moves {
            let mut new_game = game.clone();
            new_game.play_unchecked(&m);

            let eval = minimax(&new_game, depth - 1, alpha, beta, true);
            min_eval = min(min_eval, eval);
            beta = min(beta, eval);
            if beta <= alpha {
                break; // Alpha Cut-off
            }
        }
        return min_eval;
    }
}

fn get_best_move(game: &Chess, depth: u8) -> Move {
    let legal_moves = game.legal_moves();
    let mut best_move = legal_moves[0].clone();

    let maximizing = game.turn() == Color::White;

    let mut best_val = if maximizing { -100_000 } else { 100_000 };
    let mut alpha = -100_000;
    let mut beta = 100_000;

    for m in legal_moves {
        let mut new_game = game.clone();
        new_game.play_unchecked(&m);

        let val = minimax(&new_game, depth - 1, alpha, beta, !maximizing);

        if maximizing {
            if val > best_val {
                best_val = val;
                best_move = m;
            }
            alpha = max(alpha, val)
        } else {
            if val < best_val {
                best_val = val;
                best_move = m;
            }
            beta = min(beta, val);
        }
    }
    best_move
}

fn main() {
    let mut game = Chess::default();
    println!("--- RUST CHESS ENGINE ---");
    println!("Grasz biaymi. Wpisuj ruchy jak: e2e4, g1f3, a7a8q (promocja).");

    loop {
        print_board(&game);
        println!("Ocena pozycji: {}", evaluate_board(&game));

        if game.is_game_over() {
            println!("KONIEC GRY!");
            break;
        }

        if game.turn() == Color::White {
            // --- RUCH CZŁOWIEKA ---
            print!("Twój ruch: ");
            io::stdout().flush().unwrap(); // Wypchnij tekst do konsoli natychmiast

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim(); // Usuń spacje i enter

            if input == "quit" || input == "exit" {
                break;
            }

            // Parsowanie ruchu (String -> Uci -> Move)
            // 1. Najpierw parsujemy tekst do formatu UCI
            match UciMove::from_str(input) {
                Ok(uci) => {
                    // 2. Potem sprawdzamy, czy ruch jest legalny w tej pozycji
                    match uci.to_move(&game) {
                        Ok(m) => {
                            game.play_unchecked(&m);
                        }
                        Err(_) => {
                            println!("Ruch nielegalny! Spróbuj ponownie.");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Błędny format! Użyj np. e2e4");
                    continue;
                }
            }
        } else {
            // --- RUCH SILNIKA ---
            println!("Silnik myśli...");

            // Rust jest szybki - spróbujmy głębokości 5!
            let best_move = get_best_move(&game, 5);

            println!(
                "Silnik zagrał: {}",
                best_move.to_uci(shakmaty::CastlingMode::Standard)
            );
            game.play_unchecked(&best_move);
        }
    }
}
