import chess.polyglot
import chess

hasher = chess.polyglot.ZobristHasher(chess.polyglot.POLYGLOT_RANDOM_ARRAY)

hash = hasher.hash_board(chess.Board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"))

print(hash)