import chess
import random

piece_values = {
    chess.PAWN: 10,
    chess.KNIGHT: 30,
    chess.BISHOP: 30,
    chess.ROOK: 50,
    chess.QUEEN: 90,
    chess.KING: 900
}

pawntable = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, -20, -20, 10, 10,  5,
    5, -5, -10,  0,  0, -10, -5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5,  5, 10, 25, 25, 10,  5,  5,
    10, 10, 20, 30, 30, 20, 10, 10,
    50, 50, 50, 50, 50, 50, 50, 50,
    0,  0,  0,  0,  0,  0,  0,  0]

knightstable = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20,   0,   5,   5,   0, -20, -40,
    -30,   5,  10,  15,  15,  10,   5, -30,
    -30,   0,  15,  20,  20,  15,   0, -30,
    -30,   5,  15,  20,  20,  15,   5, -30,
    -30,   0,  10,  15,  15,  10,   0, -30,
    -40, -20,   0,   0,   0,   0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50]

bishopstable = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10,   0,   0,   0,   0,   0,   0, -10,
    -10,   0,   5,  10,  10,   5,   0, -10,
    -10,   5,   5,  10,  10,   5,   5, -10,
    -10,   0,  10,  10,  10,  10,   0, -10,
    -10,  10,  10,  10,  10,  10,  10, -10,
    -10,   5,   0,   0,   0,   0,   5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20]

rookstable = [
      0,  0,  0,  0,  0,  0,  0,  0,
      5, 10, 10, 10, 10, 10, 10,  5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
     -5,  0,  0,  0,  0,  0,  0, -5,
      0,  0,  0,  5,  5,  0,  0,  0]

queenstable = [
    -20, -10, -10, -5, -5, -10, -10, -20,
    -10,   0,   0,  0,  0,   0,   0, -10,
    -10,   0,   5,  5,  5,   5,   0, -10,
     -5,   0,   5,  5,  5,   5,   0, -5,
      0,   0,   5,  5,  5,   5,   0, -5,
    -10,   5,   5,  5,  5,   5,   0, -10,
    -10,   0,   5,  0,  0,   0,   0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20]

kingstable = [
     20,  30,  10,   0,   0,  10,  30,  20,
     20,  20,   0,   0,   0,   0,  20,  20,
    -10, -20, -20, -20, -20, -20, -20, -10,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -30, -40, -40, -50, -50, -40, -40, -30]


# Simple rating the board by pieces values
def evaluate_board(board):
    if board.is_checkmate():
        if board.turn: return -9999
        else: return 9999
    
    score = 0

    
    for square in chess.SQUARES:
        piece = board.piece_at(square)
        if piece is None: continue
        
        # Base value
        score += piece_values[piece.piece_type] if piece.color == chess.WHITE else -piece_values[piece.piece_type]

        # Positional value
        if piece.piece_type == chess.PAWN:
            if piece.color == chess.WHITE:
                score += pawntable[square]
            else:
                # Mirror the square index for Black (63 - square)
                score -= pawntable[chess.square_mirror(square)]
        
        elif piece.piece_type == chess.KNIGHT:
            if piece.color == chess.WHITE:
                score += knightstable[square]
            else:
                score -= knightstable[chess.square_mirror(square)]

        elif piece.piece_type == chess.BISHOP:
            if piece.color == chess.WHITE:
                score += bishopstable[square]
            else:
                score -= bishopstable[chess.square_mirror(square)]

        elif piece.piece_type == chess.ROOK:
            if piece.color == chess.WHITE:
                score += rookstable[square]
            else:
                score -= rookstable[chess.square_mirror(square)]

        elif piece.piece_type == chess.QUEEN:
            if piece.color == chess.WHITE:
                score += queenstable[square]
            else:
                score -= queenstable[chess.square_mirror(square)]

        elif piece.piece_type == chess.KING:
            if piece.color == chess.WHITE:
                score += kingstable[square]
            else:
                score -= kingstable[chess.square_mirror(square)]
    
    return score

def get_random_move(board):
    """
    The 'Dummy' Brain.
    It looks at all legal moves and picks one at random.
    """
    legal_moves = list(board.legal_moves)
    return random.choice(legal_moves)



def minimax(board, depth, alpha, beta, maximizing_player):
    if depth == 0 or board.is_game_over():
        return evaluate_board(board)

    legal_moves = list(board.legal_moves)
    legal_moves.sort(key=lambda move: board.is_capture(move), reverse=True)

    if maximizing_player:
        max_eval = -float('inf')
        for move in legal_moves:
            board.push(move)
            eval = minimax(board, depth-1, alpha, beta, False)
            board.pop()
            max_eval = max(max_eval, eval)
            alpha = max(alpha, eval)
            if beta <= alpha:
                break
        return max_eval
    else:
        min_eval = float('inf')
        for move in legal_moves:
            board.push(move)
            eval = minimax(board, depth-1, alpha, beta, True)
            board.pop()
            min_eval = min(min_eval, eval)
            beta = min(beta, eval)
            if beta <= alpha:
                break
        return min_eval

def get_best_move(board, depth):
    best_move = None
    max_eval = -float('inf')
    min_eval = float('inf')
    
    # We need to run the first layer of the loop here to find WHICH move is best
    # (The minimax function above only returns the Score, not the Move)
    legal_moves = list(board.legal_moves)
    legal_moves.sort(key=lambda move: board.is_capture(move), reverse=True)

    is_maximizing = (board.turn == chess.WHITE)
    alpha = -float('inf')
    beta = float('inf')
    
    for move in legal_moves:
        board.push(move)
        # Call minimax for the opponent's turn
        eval = minimax(board, depth - 1, alpha, beta, not is_maximizing)
        board.pop()
        
        if is_maximizing:
            if eval > max_eval:
                max_eval = eval
                best_move = move
            alpha = max(alpha, eval)
        else:
            if eval < min_eval:
                min_eval = eval
                best_move = move
            beta = min(beta, eval)
            
    return best_move






def play_game():
    # Initialize the board
    board = chess.Board()

    print("Welcome to your Chess Engine!")
    print("Format moves like: e2e4, g1f3, a7a5")
    print("-----------------------------------")

    while not board.is_game_over():
        # Print the board text representation
        print(board)
        print("\n")
        print(evaluate_board(board))

        if board.turn == chess.WHITE:
            # --- HUMAN TURN (WHITE) ---
            while True:
                try:
                    move_str = input("Your move (White): ")
                    # specific command to quit
                    if move_str == "quit":
                        return
                        
                    # Parse the move string into a Move object
                    move = chess.Move.from_uci(move_str)

                    if move in board.legal_moves:
                        board.push(move)
                        break # valid move, exit the input loop
                    else:
                        print("Illegal move. Try again.")
                except ValueError:
                    print("Invalid format. Use UCI format (e.g., e2e4).")

        else:
            # --- ENGINE TURN (BLACK) ---
            print("Engine is thinking...")
            
            # 1. Get the move
            engine_move = get_best_move(board, 5)
            
            # 2. Make the move
            board.push(engine_move)
            print(f"Engine played: {engine_move.uci()}")
            print("-----------------------------------")

    # Game Over
    print("Game Over!")
    print(f"Result: {board.result()}")

if __name__ == "__main__":
    play_game()