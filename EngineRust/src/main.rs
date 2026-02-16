use rand::seq::SliceRandom;
use shakmaty::{Chess, Color, Move, Position, Role};

fn main() {
    // 1. Setup the board
    let mut game = Chess::default();
    let mut rng = rand::thread_rng();

    println!("--- RUST ENGINE ONLINE ---");

    // 2. Play 10 random moves
    for i in 1..=10 {
        if game.is_game_over() {
            println!("Game over!");
            break;
        }

        // Generate legal moves
        let legal_moves = game.legal_moves();

        // Pick one randomly
        if let Some(random_move) = legal_moves.choose(&mut rng) {
            // Print what happened
            println!(
                "Move {}: {}",
                i,
                random_move.to_uci(shakmaty::CastlingMode::Standard)
            );

            // Make the move on the board
            game.play_unchecked(random_move);
        } else {
            println!("No legal moves available.");
            break;
        }
    }
}
