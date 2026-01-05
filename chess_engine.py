from time import perf_counter
import chess
import chess.svg
import sys

    

class ChessEngine:
    def __init__(self, depth: int = 5, color:chess.Color=chess.WHITE):
        self.depth = depth  # Default search depth
        self.color = color
        pass

    def __static_evaluate_board___(self, board: chess.Board) -> int:
        evaluation = 0

        evaluation += self.__material_count__(board)
        evaluation += self.__promotion_rules__(board)
        evaluation += self.__center_control_evaluation__(board)
        evaluation += self.__control_evaluation__(board)

        if board.is_game_over():
            evaluation = self.__game_ending_evaluation__(board) # Check for game ending conditions

        return evaluation

    def __game_ending_evaluation__(self, board: chess.Board) -> int:
        if board.is_checkmate():
            if board.turn == self.color:
                return -9999  # Checkmate against us
            else:
                return 9999   # Checkmate for us
        elif board.is_stalemate() or board.is_insufficient_material() or board.can_claim_fifty_moves() or board.can_claim_threefold_repetition():
            return 0  # Draw
        return 0

    def __material_count__(self, board: chess.Board) -> int:
        piece_values = {
            chess.PAWN: 1,
            chess.KNIGHT: 3,
            chess.BISHOP: 3,
            chess.ROOK: 5,
            chess.QUEEN: 9,
            chess.KING: 0
        }
        material = 0
        for piece_type in piece_values:
            material += len(board.pieces(piece_type, self.color)) * piece_values[piece_type] # Add our material
            material -= len(board.pieces(piece_type, not self.color)) * piece_values[piece_type] # Subtract opponent's material
        return material
    
    def __promotion_rules__(self, board: chess.Board) -> int:
        promotion_score = 0

        pawns = board.pieces(chess.PAWN, chess.WHITE)

        for pawn in pawns:
            promotion_score += (chess.square_rank(pawn)-1) * 0.1  # Closer to 7th rank is better

        pawns = board.pieces(chess.PAWN, chess.BLACK)

        for pawn in pawns:
            promotion_score -= ((6 - chess.square_rank(pawn))) * 0.1  # Closer to 7th rank is better

        return promotion_score

    def __center_control_evaluation__(self, board: chess.Board) -> int:
        center_squares = [chess.D4, chess.D5, chess.E4, chess.E5]
        control_score = 0

        for square in center_squares:
            attackers_white = board.attackers(chess.WHITE, square)
            attackers_black = board.attackers(chess.BLACK, square)

            control_score += (len(attackers_white) - len(attackers_black)) * 0.1  # Each attacker contributes 0.2 to the score

        return control_score

    def __control_evaluation__(self, board: chess.Board) -> int:
        control_evaluation = 0

        for square in chess.SQUARES:
            attackers_white = board.attackers(chess.WHITE, square)
            attackers_black = board.attackers(chess.BLACK, square)

            control_evaluation += (len(attackers_white) - len(attackers_black))
        
        control_evaluation *= 0.01  # Scale down the control evaluation

        return control_evaluation

    def __generate_moves_tree__(self, board: chess.Board, depth:int=5, alpha:float =float('-inf'), beta:float =float('inf'), is_black:bool=False) -> tuple[int, chess.Move|None]:

        if depth == 0 or board.is_game_over():
            score = self.__static_evaluate_board___(board)
            #print(is_black, score)
            return -score if is_black else score, None
        
        max_eval = float("-inf")
        best_move = None

        for move in board.legal_moves:
            board.push(move)

            val, _ = self.__generate_moves_tree__(board, depth -1, -beta, -alpha, is_black)
            val = -val

            board.pop()

            if max_eval < val:
                max_eval = val
                best_move = move


            alpha = max(alpha, val)

            if alpha >= beta:
                break

        return max_eval, best_move

    def evaluate_board(self, board: chess.Board) -> int:
        eval = self.__generate_moves_tree__(board, self.depth, is_black=not self.color)
        return eval if self.color == chess.WHITE else -eval

    def find_best_move(self, board: chess.Board) -> str:
        _, best_move = self.__generate_moves_tree__(board, self.depth, is_black=not self.color)

        return best_move

    def find_best_move_eval(self, board: chess.Board) -> int:
        eval, best_move = self.__generate_moves_tree__(board, self.depth, is_black=not self.color)

        if self.color == chess.BLACK:
            eval = -eval
            
        return eval, best_move


def update_visual(board):
    with open("board_view.html", "w") as f:
        # SVG wrapped in basic HTML with auto-refresh every 2 seconds
        html = f'<html><head><meta http-equiv="refresh" content="2"></head>'
        html += f'<body>{chess.svg.board(board, size=400)}</body></html>'
        f.write(html)


if False:  # Change to True to run the test code


    board = chess.Board("p4ppp/P1p1p3/R2p3P/3P1P2/2N1P3/1PP4R/2B1K3/8 w KQkq - 0 1")
    engine = ChessEngine(2, chess.WHITE)

    print("Initial board:")
    print(board)

    start_time = perf_counter()

    best_move = engine.find_best_move(board)
    print("Best move:", best_move)

    end_time = perf_counter()

    print(f"Time taken: {end_time - start_time:.4f} seconds")

    print(engine.__static_evaluate_board___(board),board.is_stalemate())

    try:
        open("test.svg", "w").write( 
            chess.svg.board(board)
        )
    except Exception as e:
        print("Could not write SVG file:", e)

    while True:
        if board.is_game_over():
            break
        start_time = perf_counter()
        best_move = engine.find_best_move(board)
        end_time = perf_counter()
        print("Best move:", best_move)
        print(f"Time taken: {end_time - start_time:.4f} seconds")
        board.push(best_move)
        update_visual(board)

        board.push_uci(input("Enter your move: "))


    #print_tree(root)

if __name__ == "__main__":
    if sys.argv[1] == "find_best_move":
        fen = sys.argv[2:8]
        board = chess.Board(" ".join(fen))
        color = chess.WHITE if sys.argv[8].lower() == "w" else chess.BLACK
        depth = int(sys.argv[9])
        engine = ChessEngine(depth, color)
        best_move = engine.find_best_move_eval(board)
        print(best_move.uci())

    elif sys.argv[1] == "find_best_move_eval":
        fen = sys.argv[2:8]
        board = chess.Board(" ".join(fen))
        color = chess.WHITE if sys.argv[8].lower() == "w" else chess.BLACK
        depth = int(sys.argv[9])
        engine = ChessEngine(depth, color)
        eval, best_move = engine.find_best_move_eval(board)
        print(f"{best_move.uci()} {eval}")