use shakmaty::uci::UciMove;
use shakmaty::{Chess, Color, Position, Square};
use std::io::{self, Write}; // Do wczytywania inputu
use std::str::FromStr; // Do parsowania napisów // Standard UCI (np. "e2e4")

mod eval;
mod search;
mod tt;

use crate::eval::evaluate_board;
use crate::search::get_best_move;
use crate::tt::TranspositionTable; // ADD THIS

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

fn main() {
    let mut game = Chess::default();
    let mut tt = TranspositionTable::new();
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
            let best_move = get_best_move(&game, 5, &mut tt);

            println!(
                "Silnik zagrał: {}",
                best_move.to_uci(shakmaty::CastlingMode::Standard)
            );
            game.play_unchecked(&best_move);
        }
    }
}
