use crate::eval::{evaluate_board, get_piece_value};
use crate::tt::{Flag, TranspositionTable}; // Import the new module
use shakmaty::{Chess, Move, Position};

// ... keep score_move and order_moves SAME AS BEFORE ...
fn score_move(m: &Move) -> i32 {
    if m.is_capture() {
        let attacker_value = get_piece_value(m.role());
        return 10000 - attacker_value;
    }
    0
}

fn order_moves(mut moves: Vec<Move>) -> Vec<Move> {
    moves.sort_by_cached_key(|m| -score_move(m));
    moves
}
// ... end keep ...

// Quiescence (unchanged, but could use TT in future)
fn quiescence(game: &Chess, mut alpha: i32, beta: i32) -> i32 {
    let stand_pat = evaluate_board(game);
    if stand_pat >= beta {
        return beta;
    }
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let moves = game.legal_moves();
    let mut capture_moves: Vec<Move> = moves.into_iter().filter(|m| m.is_capture()).collect();
    capture_moves = order_moves(capture_moves);

    for m in capture_moves {
        let mut new_game = game.clone();
        new_game.play_unchecked(&m);
        let score = -quiescence(&new_game, -beta, -alpha);
        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}

// Updated Negamax with TT
pub fn negamax(
    game: &Chess,
    depth: u8,
    mut alpha: i32,
    mut beta: i32,
    tt: &mut TranspositionTable, // Pass the table down!
) -> i32 {
    let alpha_orig = alpha;

    // 1. TT PROBE: Check if we've seen this position
    let hash = tt.compute_hash(game);
    if let Some(entry) = tt.get(hash) {
        if entry.depth >= depth {
            match entry.flag {
                Flag::Exact => return entry.score,
                Flag::LowerBound => alpha = alpha.max(entry.score),
                Flag::UpperBound => beta = beta.min(entry.score),
            }
            if alpha >= beta {
                return entry.score;
            }
        }
    }

    if game.is_game_over() {
        if game.is_checkmate() {
            return -20000 + (depth as i32);
        }
        return 0;
    }
    if depth == 0 {
        return quiescence(game, alpha, beta);
    }

    let mut moves = game.legal_moves().into_iter().collect();
    moves = order_moves(moves);

    let mut best_score = -100_000;
    let mut best_move: Option<Move> = None; // Track best move for TT

    for m in moves {
        let mut new_game = game.clone();
        new_game.play_unchecked(&m);

        let score = -negamax(&new_game, depth - 1, -beta, -alpha, tt);

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }
        if score > alpha {
            alpha = score;
        }
        if alpha >= beta {
            break;
        }
    }

    // 2. TT STORE: Save what we learned
    let flag = if best_score <= alpha_orig {
        Flag::UpperBound // Fail Low (All moves were bad)
    } else if best_score >= beta {
        Flag::LowerBound // Fail High (We found a move too good)
    } else {
        Flag::Exact // We found the exact value
    };

    tt.store(hash, best_score, depth, flag, best_move.as_ref());

    best_score
}

pub fn get_best_move(game: &Chess, depth: u8, tt: &mut TranspositionTable) -> Move {
    let mut moves = game.legal_moves().into_iter().collect();
    moves = order_moves(moves);

    let mut best_move = moves[0].clone();
    let mut best_score = -100_000;
    let mut alpha = -100_000;
    let beta = 100_000;

    for m in moves {
        let mut new_game = game.clone();
        new_game.play_unchecked(&m);

        // Pass the TT to negamax
        let score = -negamax(&new_game, depth - 1, -beta, -alpha, tt);

        // println!(
        //     "Ruch: {} Ocena: {}",
        //     m.to_uci(shakmaty::CastlingMode::Standard),
        //     score
        // );

        if score > best_score {
            best_score = score;
            best_move = m;
        }
        if score > alpha {
            alpha = score;
        }
    }
    best_move
}
