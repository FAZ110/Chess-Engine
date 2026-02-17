use shakmaty::{Chess, Color, Position, Role};

// How big is the table? 2^20 = ~1 million entries (approx 16MB RAM)
const TT_SIZE: usize = 1 << 20;

#[derive(Clone, Copy, PartialEq)]
pub enum Flag {
    Exact,      // We know the exact score
    LowerBound, // We know score >= beta (good move)
    UpperBound, // We know score <= alpha (bad move)
}

#[derive(Clone, Copy)]
pub struct TTEntry {
    pub key: u64, // The unique ID of the position
    pub score: i32,
    pub depth: u8, // How deep did we search to get this score?
    pub flag: Flag,
    pub best_move_index: u16, // Stores the best move (compressed)
}

// The Table itself
pub struct TranspositionTable {
    entries: Vec<TTEntry>,
    // Zobrist Random Keys
    piece_keys: [[u64; 64]; 12], // 12 piece types (6 white, 6 black) * 64 squares
    side_key: u64,               // Random number for "Black to move"
}

impl TranspositionTable {
    pub fn new() -> Self {
        let mut tt = TranspositionTable {
            entries: vec![
                TTEntry {
                    key: 0,
                    score: 0,
                    depth: 0,
                    flag: Flag::Exact,
                    best_move_index: 0
                };
                TT_SIZE
            ],
            piece_keys: [[0; 64]; 12],
            side_key: 0,
        };
        tt.init_random_keys();
        tt
    }

    // A simple pseudo-random generator to fill the keys at startup
    fn init_random_keys(&mut self) {
        let mut seed: u64 = 123456789;
        let mut rand = || -> u64 {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            seed
        };

        for piece in 0..12 {
            for square in 0..64 {
                self.piece_keys[piece][square] = rand();
            }
        }
        self.side_key = rand();
    }

    // Calculate the unique ID (Hash) for a board
    pub fn compute_hash(&self, game: &Chess) -> u64 {
        let board = game.board();
        let mut hash = 0;

        for (square, piece) in board
            .occupied()
            .into_iter()
            .map(|s| (s, board.piece_at(s).unwrap()))
        {
            // Map piece to 0..11 index
            let piece_idx = match (piece.color, piece.role) {
                (Color::White, Role::Pawn) => 0,
                (Color::White, Role::Knight) => 1,
                (Color::White, Role::Bishop) => 2,
                (Color::White, Role::Rook) => 3,
                (Color::White, Role::Queen) => 4,
                (Color::White, Role::King) => 5,
                (Color::Black, Role::Pawn) => 6,
                (Color::Black, Role::Knight) => 7,
                (Color::Black, Role::Bishop) => 8,
                (Color::Black, Role::Rook) => 9,
                (Color::Black, Role::Queen) => 10,
                (Color::Black, Role::King) => 11,
            };
            hash ^= self.piece_keys[piece_idx][usize::from(square)];
        }

        if game.turn() == Color::Black {
            hash ^= self.side_key;
        }

        // (For a perfect engine, we'd also hash Castling/EnPassant, but this is enough for now!)
        hash
    }

    pub fn get(&self, hash: u64) -> Option<TTEntry> {
        let index = (hash as usize) & (TT_SIZE - 1);
        let entry = self.entries[index];
        if entry.key == hash { Some(entry) } else { None }
    }

    pub fn store(
        &mut self,
        hash: u64,
        score: i32,
        depth: u8,
        flag: Flag,
        best_move: Option<&shakmaty::Move>,
    ) {
        let index = (hash as usize) & (TT_SIZE - 1);

        // Replacement scheme: Replace if new depth is higher OR if table slot is empty/old
        if depth >= self.entries[index].depth || self.entries[index].key == 0 {
            // We only store the "to" and "from" of the move to save space,
            // or just 0 if no move. (Simplification: we won't store the move for now to keep it simple)
            self.entries[index] = TTEntry {
                key: hash,
                score,
                depth,
                flag,
                best_move_index: 0,
            };
        }
    }
}
