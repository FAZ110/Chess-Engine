use shakmaty::{Chess, Color, Position, Role};

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

pub fn get_piece_value(role: Role) -> i32 {
    match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 20000,
    }
}

pub fn evaluate_board(game: &Chess) -> i32 {
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
