from time import perf_counter
import chess
import chess.polyglot
import sys
from json import loads

class ChessEngine:
    def __init__(self):
        self.PST = {
            chess.PAWN: [
                0,  0,  0,  0,  0,  0,  0,  0,
                50, 50, 50, 50, 50, 50, 50, 50,
                10, 10, 20, 30, 30, 20, 10, 10,
                5,  5, 10, 25, 25, 10,  5,  5,
                0,  0,  0, 20, 20,  0,  0,  0,
                5, -5,-10,  0,  0,-10, -5,  5,
                5, 10, 10,-20,-20, 10, 10,  5,
                0,  0,  0,  0,  0,  0,  0,  0
            ],
            chess.KNIGHT: [
                -50,-40,-30,-30,-30,-30,-40,-50,
                -40,-20,  0,  0,  0,  0,-20,-40,
                -30,  0, 10, 15, 15, 10,  0,-30,
                -30,  5, 15, 20, 20, 15,  5,-30,
                -30,  0, 15, 20, 20, 15,  0,-30,
                -30,  5, 10, 15, 15, 10,  5,-30,
                -40,-20,  0,  5,  5,  0,-20,-40,
                -50,-40,-30,-30,-30,-30,-40,-50
            ],
            chess.BISHOP: [
                -20,-10,-10,-10,-10,-10,-10,-20,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -10,  0,  5, 10, 10,  5,  0,-10,
                -10,  5,  5, 10, 10,  5,  5,-10,
                -10,  0, 10, 10, 10, 10,  0,-10,
                -10, 10, 10, 10, 10, 10, 10,-10,
                -10,  5,  0,  0,  0,  0,  5,-10,
                -20,-10,-10,-10,-10,-10,-10,-20
            ],
            chess.ROOK: [
                0,  0,  0,  0,  0,  0,  0,  0,
                5, 10, 10, 10, 10, 10, 10,  5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                -5,  0,  0,  0,  0,  0,  0, -5,
                0,  0,  0,  5,  5,  0,  0,  0
            ],
            chess.QUEEN: [
                -20,-10,-10, -5, -5,-10,-10,-20,
                -10,  0,  0,  0,  0,  0,  0,-10,
                -10,  0,  5,  5,  5,  5,  0,-10,
                -5,  0,  5,  5,  5,  5,  0, -5,
                0,  0,  5,  5,  5,  5,  0, -5,
                -10,  5,  5,  5,  5,  5,  0,-10,
                -10,  0,  5,  0,  0,  0,  0,-10,
                -20,-10,-10, -5, -5,-10,-10,-20
            ],
            'K': [
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -30,-40,-40,-50,-50,-40,-40,-30,
                -20,-30,-30,-40,-40,-30,-30,-20,
                -10,-20,-20,-20,-20,-20,-20,-10,
                20, 20,  0,  0,  0,  0, 20, 20,
                20, 30, 10,  0,  0, 10, 30, 20
            ],
            'K_ENDGAME': [
                -50,-40,-30,-20,-20,-30,-40,-50,
                -30,-20,-10,  0,  0,-10,-20,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 30, 40, 40, 30,-10,-30,
                -30,-10, 20, 30, 30, 20,-10,-30,
                -30,-30,  0,  0,  0,  0,-30,-30,
                -50,-30,-30,-30,-30,-30,-30,-50
            ]
        }
        self.piece_values = {
            chess.PAWN: 100,
            chess.KNIGHT: 300,
            chess.BISHOP: 310,
            chess.ROOK: 500,
            chess.QUEEN: 900,
            chess.KING: 0
        }
        self.piece_symbols = {
            chess.PAWN: 'p',
            chess.KNIGHT: 'n',
            chess.BISHOP: 'b',
            chess.ROOK: 'r',
            chess.QUEEN: 'q',
            chess.KING: 'k'
        }

        self.hasher = chess.polyglot.ZobristHasher(chess.polyglot.POLYGLOT_RANDOM_ARRAY) 
        self.position_stack = []
        self.transportation_table = {}
        
    def __static_evaluate_board__(self, board:chess.Board):
        evaluation = 0
        
        # Map piece types to values for speed
        piece_values = self.piece_values 
        
        is_endgame = self.__is_endgame__(board)

        # Loop over all 64 squares exactly once
        for square, piece in board.piece_map().items():
            val = piece_values[piece.piece_type]

            if piece.color == chess.WHITE:
                if piece.piece_type == chess.KING:
                    if is_endgame:
                        evaluation += self.PST['K_ENDGAME'][square]
                        continue
                    else:
                        evaluation += self.PST['K'][square]
                        continue
                evaluation += val
                # Use the square directly for PST
                evaluation += self.PST[piece.piece_type][square]
            else:
                if piece.piece_type == chess.KING:
                    if is_endgame:
                        evaluation -= self.PST['K_ENDGAME'][chess.square_mirror(square)]
                        continue
                    else:
                        evaluation -= self.PST['K'][chess.square_mirror(square)]
                        continue
                evaluation -= val
                # Flip the square for Black's perspective
                evaluation -= self.PST[piece.piece_type][chess.square_mirror(square)]

        return evaluation
    
    def __generate_moves_tree__(self, board: chess.Board, depth:int=5, alpha:float =float('-inf'), beta:float =float('inf'), color:chess.Color=chess.WHITE, hint_move:chess.Move|None=None, hash:int=None) -> tuple[int, chess.Move|None]:
        if hash is None:
            hash = self.hasher.hash_board(board)

        if board.is_checkmate():
            if board.turn == chess.WHITE:
                return -999999, None  # Checkmate for Black ( if the turn is white then black just checkmated White )
            else:
                return 999999, None   # Checkmate for White
        elif board.is_stalemate() or self.__three_fold_repetition__(hash) or board.can_claim_fifty_moves() or board.is_insufficient_material():
            return 0, None  # Draw

        if hash in self.transportation_table:
            entry = self.transportation_table[hash]
            if entry['depth'] >= depth:
                if entry['flag'] == "EXACT": return entry['value'], entry['move']
                if entry['flag'] == "LOWER": alpha = max(alpha, entry['value'])
                if entry['flag'] == "UPPER": beta = min(beta, entry['value'])
                if alpha >= beta:
                    return entry['value'], entry['move']

        if (depth <= 0 or board.is_game_over()):
            score = self.__static_evaluate_board__(board)

            self.__make_transportation_table__(hash, score, None, depth, "EXACT")

            return score, None
        
        best_move = None

        legal_moves = list(board.legal_moves)  
        legal_moves.sort(key=lambda move: self.__order_moves__(board, move, color), reverse=True)

        if hint_move and hint_move in legal_moves:
            legal_moves.remove(hint_move)
            legal_moves.insert(0, hint_move) # Puts it right at the front

        cutoff = False
        
        if color:
            best_eval = float("-inf")
            for move in legal_moves:
                board.push(move)
                move_hash = self.hasher.hash_board(board)
                self.position_stack.append(move_hash)
                eval, _ = self.__generate_moves_tree__(board, depth-1, alpha, beta, not color, None, move_hash)
                self.position_stack.pop()
                board.pop()
                
                if eval > best_eval:
                    best_eval = eval
                    best_move = move

                alpha = max(alpha, eval)

                if alpha >= beta:
                    self.__make_transportation_table__(hash, best_eval, best_move, depth, "LOWER")
                    cutoff = True
                    break
        else:
            best_eval = float("inf")
            for move in legal_moves:
                board.push(move)
                move_hash = self.hasher.hash_board(board)
                self.position_stack.append(move_hash)
                eval, _ = self.__generate_moves_tree__(board, depth-1, alpha, beta, not color, None, move_hash)
                self.position_stack.pop()
                board.pop()

                if eval < best_eval:
                    best_eval = eval
                    best_move = move

                beta = min(beta, eval)

                if beta <= alpha:
                    self.__make_transportation_table__(hash, best_eval, best_move, depth, "UPPER")
                    cutoff = True

                    break

        if not cutoff:
            self.__make_transportation_table__(hash, best_eval, best_move, depth, "EXACT")
            pass

        return best_eval, best_move

    def __make_transportation_table__(self, hash, value, move, depth, flag="EXACT"):
        self.transportation_table[hash] = {
            'value':value,
            'move':move,
            'depth':depth,
            'flag':flag
        }

    def __quiescence_evaluation__(self, board: chess.Board, alpha: int, beta: int) -> int:
        captures = [move for move in board.legal_moves if board.is_capture(move)]
        if captures == []:
            return self.__static_evaluate_board___(board)
        
        
        best_evaluation = float('-inf')

        for capture in captures:
            board.push(capture)
            evaluation = self.__quiescence_evaluation__(board, -beta, -alpha)
            board.pop()

            evaluation = -evaluation

            best_evaluation = max(evaluation, best_evaluation)

            alpha = max(evaluation, alpha)

            if alpha >= beta:
                return best_evaluation
            

    def evaluate_board(self, board: chess.Board, depth: int=3, color: chess.Color=chess.WHITE) -> int:
        self.position_stack.append(self.hasher.hash_board(board))
        eval, _ = self.__generate_moves_tree__(board, depth, is_black=color)
        self.position_stack.pop()
        return eval

    def find_best_move(self, board: chess.Board, depth: int=3, color: chess.Color=chess.WHITE) -> str:
        self.position_stack.append(self.hasher.hash_board(board))
        _, best_move = self.__generate_moves_tree__(board, depth, is_black=color)
        self.position_stack.pop()
        return best_move

    def find_best_move_eval(self, board: chess.Board, depth: int=3, color: chess.Color=chess.WHITE) -> tuple[int, chess.Move]:
        self.position_stack.append(self.hasher.hash_board(board))

        best_move = None

        for i in range(1, depth+1):
            eval, best_move = self.__generate_moves_tree__(board, i, color=color, hint_move=best_move)

        self.position_stack.pop()

        return eval, best_move

    def __is_endgame__ (self, board: chess.Board) -> bool:
        pieces = board.piece_map()

        minor_pieces = 0

        queens = 0

        for _, piece in pieces.items():
            if piece.piece_type == chess.BISHOP or piece.piece_type == chess.KNIGHT:
                minor_pieces += 1
            elif piece.piece_type == chess.QUEEN:
                queens += 1
        
        return (queens <= 1 and minor_pieces <= 2) or (queens <= 0 and minor_pieces <= 3)

    def __order_moves__(self, board: chess.Board, move: chess.Move, color: chess.Color) -> int:
        score=0

        captured_piece = board.piece_at(move.to_square)

        if captured_piece:
            score += self.piece_values[captured_piece.piece_type * 10] - self.piece_values[board.piece_at(move.from_square).piece_type]

        return score

    def __three_fold_repetition__(self, current_hash):
        if len(self.position_stack) < 3:
            return False
        
        count = self.position_stack.count(current_hash)

        if count >= 2:
            return True

        return False

def process_input(input_string: str, engine: ChessEngine) -> None:
    if input_string == "quit":
        exit(0)
    
    input_string = loads(input_string)

    command = input_string["command"]
    fen = input_string["fen"]
    color = chess.WHITE if input_string["color"] == "w" else chess.BLACK
    depth = input_string["depth"]

    board = chess.Board(fen)

    if command == "move_eval":
        evaluation, best_move = engine.find_best_move_eval(board, depth, color)
        print(f"{board.san(best_move)} {evaluation}", flush=True)

    elif command == "eval":
        evaluation = engine.evaluate_board(board, depth, color)
        print(f"{evaluation}", flush=True)
    elif command == "test":
        print("testing")
        estart_time = perf_counter()
        engine.__static_evaluate_board___(board)
        eend_time = perf_counter()

        hstart_time = perf_counter()
        engine.hasher.hash_board(board)
        hend_time = perf_counter()

        print(f"Time taken: {eend_time - estart_time:.4f} seconds for evaluation, {hend_time - hstart_time:.4f} seconds for hashing,")

if False:  # Change to True to run the test code
    board = chess.Board()
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
    engine = ChessEngine()

    while True:
        try:
            line = sys.stdin.readline()
            if line:
                process_input(line, engine)
        except EOFError:
            break
        except ValueError:
            break