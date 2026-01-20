use cozy_chess::{self as chess};
use serde_json::Result;
use std::io::{stdin};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Request{
    command: String,
    moves: Vec<String>,
    color: String,
    depth: i8
}

pub const MAX_PHASE: i32 = 24; // Total phase value at the start of the game

pub const PHASE_WEIGHTS: [i32; 6] = [
    0,   // PAWN
    1,   // KNIGHT
    1,   // BISHOP
    2,   // ROOK
    4,   // QUEEN
    0    // KING
];

// From now on MG = Middlegame, EG = Endgame

pub const MG_PIECE_VALUES: [i32; 6] = [
    100, // PAWN
    300, // KNIGHT
    300, // BISHOP
    500, // ROOK
    900, // QUEEN
    0 // KING
];

pub const EG_PIECE_VALUES: [i32; 6] = [
    128, // Pawn
    213, // Knight
    276, // Bishop
    441, // Rook
    825, // Queen
    0    // King (King has no material value)
];

pub const MG_PST: [[i32; 64]; 6] = [
    // 0: PAWN (Control center, don't block king, push for space)
    [
        0,  0,  0,  0,  0,  0,  0,  0,      // Rank 1
        5, 10, 10,-20,-20, 10, 10,  5,      // Rank 2
        5, -5,-10,  0,  0,-10, -5,  5,      // Rank 3
        0,  0,  0, 20, 20,  0,  0,  0,      // Rank 4
        5,  5, 10, 25, 25, 10,  5,  5,      // Rank 5
        10, 10, 20, 30, 30, 20, 10, 10,     // Rank 6
        50, 50, 50, 50, 50, 50, 50, 50,     // Rank 7
        0,  0,  0,  0,  0,  0,  0,  0       // Rank 8
    ],
    // 1: KNIGHT (Aggressive center control - better than EG version)
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  5, 20, 30, 30, 20,  5,-30,
        -30,  5, 20, 30, 30, 20,  5,-30,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50
    ],
    // 2: BISHOP (Control long diagonals, avoid corners)
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5, 10, 15, 15, 10,  5,-10,
        -10,  5, 10, 15, 15, 10,  5,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -20,-10,-10,-10,-10,-10,-10,-20
    ],
    // 3: ROOK (Control open files and 7th rank)
    [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        5, 10, 15, 15, 15, 15, 10,  5,
        0,  0,  0,  5,  5,  0,  0,  0
    ],
    // 4: QUEEN (Avoid early centralization, stay safe but active)
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -5,  0,  5,  5,  5,  5,  0, -5,
        -5,  0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ],
    // 5: KING (Safety first - Keep in the corner for Midgame)
    [
        20, 30, 10,  0,  0, 10, 30, 20,
        20, 20,  0,  0,  0,  0, 20, 20,
        -10,-20,-20,-20,-20,-20,-20,-10,
        -20,-30,-30,-40,-40,-30,-30,-20,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30,
        -30,-40,-40,-50,-50,-40,-40,-30
    ]
];

pub const EG_PST: [[i32; 64]; 6] = [
    // 0: PAWN (Aggressive push to promotion)
    [
        0,  0,  0,  0,  0,  0,  0,  0,      // Rank 1
        0,  0,  0,  0,  0,  0,  0,  0,      // Rank 2
        5,  5,  5,  5,  5,  5,  5,  5,      // Rank 3
        5,  5,  5,  5,  5,  5,  5,  5,      // Rank 4
        10, 10, 10, 10, 10, 10, 10, 10,     // Rank 5
        20, 20, 20, 20, 20, 20, 20, 20,     // Rank 6
        50, 50, 50, 50, 50, 50, 50, 50,     // Rank 7
        0,  0,  0,  0,  0,  0,  0,  0       // Rank 8
    ],
    // 1: KNIGHT (Centralization, avoiding corners)
    [
        -50,-40,-30,-30,-30,-30,-40,-50,
        -40,-20,  0,  5,  5,  0,-20,-40,
        -30,  5, 10, 15, 15, 10,  5,-30,
        -30,  0, 15, 20, 20, 15,  0,-30,
        -30,  5, 15, 20, 20, 15,  5,-30,
        -30,  0, 10, 15, 15, 10,  0,-30,
        -40,-20,  0,  0,  0,  0,-20,-40,
        -50,-40,-30,-30,-30,-30,-40,-50
    ],
    // 2: BISHOP (Active diagonals)
    [
        -20,-10,-10,-10,-10,-10,-10,-20,
        -10,  5,  0,  0,  0,  0,  5,-10,
        -10, 10, 10, 10, 10, 10, 10,-10,
        -10,  0, 10, 10, 10, 10,  0,-10,
        -10,  5,  5, 10, 10,  5,  5,-10,
        -10,  0,  5, 10, 10,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10,-10,-10,-10,-10,-20
    ],
    // 3: ROOK (7th rank dominance and mobility)
    [
        0,  0,  0,  5,  5,  0,  0,  0,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        -5,  0,  0,  0,  0,  0,  0, -5,
        5, 10, 10, 10, 10, 10, 10,  5,
        0,  0,  0,  0,  0,  0,  0,  0
    ],
    // 4: QUEEN (Central pressure)
    [
        -20,-10,-10, -5, -5,-10,-10,-20,
        -10,  0,  5,  0,  0,  0,  0,-10,
        -10,  5,  5,  5,  5,  5,  0,-10,
        0,  0,  5,  5,  5,  5,  0, -5,
        -5,  0,  5,  5,  5,  5,  0, -5,
        -10,  0,  5,  5,  5,  5,  0,-10,
        -10,  0,  0,  0,  0,  0,  0,-10,
        -20,-10,-10, -5, -5,-10,-10,-20
    ],
    // 5: KING (The "Active King" logic)
    [
        -50,-30,-30,-30,-30,-30,-30,-50,
        -30,-30,  0,  0,  0,  0,-30,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 30, 40, 40, 30,-10,-30,
        -30,-10, 20, 30, 30, 20,-10,-30,
        -30,-20,-10,  0,  0,-10,-20,-30,
        -50,-40,-30,-20,-20,-30,-40,-50
    ]
];

struct ScoredMove {
    mv: chess::Move,
    score: i32
}

#[derive(Clone, Copy, PartialEq)]
pub enum TtFlags {
    NONE,
    EXACT,
    LOWER,
    UPPER
}

#[derive(Clone, Copy)]
struct TtEntry {
    key: u64,
    score: i32,
    depth: i8,
    flag: TtFlags,
    best_move: Option<chess::Move>
}

pub struct Tt {
    table: Box<[TtEntry]>,
    mask: u64, // Used for fast index calculation
}

impl Tt {
    pub fn new(size: u64) -> Tt {
        Tt {
            table: vec![TtEntry::default(); size as usize].into_boxed_slice(),
            mask: size - 1
        }
    }

    fn probe(&self, hash_key: u64, depth: i8) -> Option<&TtEntry> {
        let index = (hash_key & self.mask) as usize;
        let entry = &self.table[index];

        // 1. Key Match: Is this the same board position?
        // 2. Depth Check: Is the stored data as deep or deeper than what we need?
        if entry.key == hash_key && entry.depth >= depth {
            return Some(entry);
        }
        
        None // Miss: Slot is empty, wrong position, or depth too shallow
    }

    pub fn store(&mut self, hash_key: u64, depth: i8, score: i32, flag: TtFlags, best_move: Option<chess::Move>) {
        let index = (hash_key & self.mask) as usize;
        
        // Simple "Always Replace" strategy (fastest)
        // More advanced engines might check if entry.depth > depth before replacing.
        self.table[index] = TtEntry {
            key: hash_key,
            depth,
            score,
            flag,
            best_move
        };
    }
}

impl Default for TtEntry {
    fn default() -> TtEntry {
        TtEntry {
            key: 0,
            score: 0,
            depth: 0,
            flag: TtFlags::NONE,
            best_move: None
        }
    }
}


struct Engine {
    transposition_table: Tt,
}

impl Engine {

    fn new() -> Engine {
        return Engine {
            transposition_table: Tt::new(1_048_576)
        };
    }

    fn calculate_game_phase(&self, board: &chess::Board) -> i32 {
        // The chess crate's `pieces` method returns a BitBoard.
        // .popcnt() counts the number of set bits (pieces) efficiently.
        
        let knights = board.pieces(chess::Piece::Knight).len();
        let bishops = board.pieces(chess::Piece::Bishop).len();
        let rooks   = board.pieces(chess::Piece::Rook).len();
        let queens  = board.pieces(chess::Piece::Queen).len();

        // Calculate total phase
        let phase = (knights * 1) + (bishops * 1) + (rooks * 2) + (queens * 4);

        // We use .min to ensure we never exceed 24 (though standard chess setup is exactly 24)
        std::cmp::min(phase as i32, MAX_PHASE)
    }
    


    fn static_evaluate_board(&self, board: &chess::Board) -> i32 {
        let mut mg_evaluation = 0;
        let mut eg_evaluation = 0;

        for piece_type in chess::Piece::ALL {
            
            for piece in board.colored_pieces(chess::Color::White, piece_type) { 
                mg_evaluation += MG_PIECE_VALUES[piece_type as usize];
                mg_evaluation += MG_PST[piece_type as usize][piece as usize];
                eg_evaluation += EG_PIECE_VALUES[piece_type as usize];
                eg_evaluation += EG_PST[piece_type as usize][piece as usize];
            }
            for piece in board.colored_pieces(chess::Color::Black, piece_type) {
                mg_evaluation -= MG_PIECE_VALUES[piece_type as usize];
                mg_evaluation -= MG_PST[piece_type as usize][piece.flip_rank() as usize];
                eg_evaluation -= EG_PIECE_VALUES[piece_type as usize];
                eg_evaluation -= EG_PST[piece_type as usize][piece.flip_rank() as usize];
            }
        } 

        let phase = self.calculate_game_phase(board);

        let result = ((mg_evaluation * phase) + (eg_evaluation * (MAX_PHASE - phase))) / MAX_PHASE;

        return if board.side_to_move() == chess::Color::White { result } else { -result };
    }

    fn quiescence_search(&mut self, board: &mut chess::Board, mut alpha: i32, beta: i32) -> i32 {
        let stand_pat = self.static_evaluate_board(board);

        if stand_pat >= beta {
            return beta; // Beta cutoff
        }

        let promotion_rank = match board.side_to_move() {
            chess::Color::White => chess::Rank::Seventh,
            chess::Color::Black => chess::Rank::Second
        };

        let pawns_on_promotion_rank = board.colored_pieces(board.side_to_move(), chess::Piece::Pawn)
            .into_iter()
            .filter(|&sq| sq.rank() == promotion_rank)
            .count();

        if pawns_on_promotion_rank == 0 {
            let delta = 1100; // Max promotion gain (Queen + 2 Pawns)
            if stand_pat + delta < alpha{
                return alpha;
            }
        }

        if stand_pat > alpha {
            // Alpha improvement
            alpha = stand_pat;
        }

        let mut capture_moves = Vec::new();
        let phase = self.calculate_game_phase(board);

        board.generate_moves(|moves: chess::PieceMoves| {
            capture_moves.extend(moves.into_iter().filter_map(|mv: chess::Move| {
                let victim:Option<chess::Color> = board.color_on(mv.to);
                // 1. Normal Capture (Piece on target square)
                if victim.is_some() && victim.unwrap() != board.side_to_move() {
                    return Some(ScoredMove{mv, score: self.order_moves(board, mv, None, phase)});
                }
                
                // 2. En Passant Capture (Target is the EP square)
                // Note: You might need to check if the moving piece is a pawn, 
                // but usually only pawns can move to the EP square anyway.
                if let Some(ep_file) = board.en_passant() {
                    let ep_rank = match board.side_to_move() {
                        chess::Color::White => chess::Rank::Sixth, // Rank 6 (0-indexed)
                        chess::Color::Black => chess::Rank::Third  // Rank 3 (0-indexed)
                    };

                    if mv.to == chess::Square::new(ep_file, ep_rank) {
                        return Some(ScoredMove{mv, score: self.order_moves(board, mv, None, phase)});
                    }
                }

                None
            }));
            false
        });

        capture_moves.sort_unstable_by(|a, b| b.score.cmp(&a.score));

        for scored_move in capture_moves {
            let mut next_board = board.clone();
            next_board.play(scored_move.mv);

            let score = -self.quiescence_search(&mut next_board, -beta, -alpha);

            if score >= beta {
                return beta; // Beta cutoff
            }

            if score > alpha {
                alpha = score; // Alpha improvement
            }
        }

        return alpha;
    }   
    

    fn search(&mut self, board: &mut chess::Board, depth: i8, color: chess::Color, mut alpha: i32, mut beta: i32, history: &Vec<u64>, path: &mut Vec<u64>) -> (Option<chess::Move>, i32) {
        let hash_key = board.hash();

        if self.is_repetition(history, path, hash_key) && path.len() > 0 {
            return (None, 0);
        }

        let entry = self.transposition_table.probe(hash_key, depth);
        if !entry.is_none() {
            let entry = entry.unwrap();
            if entry.depth >= depth {
                if entry.flag == TtFlags::EXACT {
                    return (entry.best_move, entry.score);
                } else if entry.flag == TtFlags::LOWER && entry.score > alpha {
                    alpha = entry.score;

                } else if entry.flag == TtFlags::UPPER && entry.score < beta {
                    beta = entry.score;
                }

                if alpha >= beta {
                    return (entry.best_move, entry.score);
                }
                
            }
        }

        let tt_entry = self.transposition_table.probe(hash_key, 0); // Get entry even if depth is low
        let tt_move = tt_entry.map(|e| e.best_move);

        if depth <= 0 {
            return (None, self.static_evaluate_board(board));
        }

        let mut legal_moves: Vec<ScoredMove> = Vec::new();

        let phase = self.calculate_game_phase(board);

        board.generate_moves(|moves| {
            legal_moves.extend(moves.into_iter().map(|mv| ScoredMove {
                mv,
                score: self.order_moves(board, mv, tt_move.unwrap_or(None), phase)
            }));

           false 
        });

        if legal_moves.len() == 0 {
            if board.checkers().is_empty(){
                return (None, 0);
            }
            let checkmate_score = 9999999 + (depth as i32);
            return (None, -checkmate_score);
        }

        legal_moves.sort_unstable_by(|a, b| b.score.cmp(&a.score));
        
        let mut best_move: Option<chess::Move> = None;
        let mut best_score: i32 = i32::MIN;

        let mut cutoff = false;

        let original_alpha = alpha;

        path.push(hash_key);

        for scored_move in legal_moves {
            let mut next_board = board.clone();
            next_board.play(scored_move.mv);

            let (_, mut score) = self.search(&mut next_board, depth - 1, !color, -beta, -alpha, history, path);

            score = -score;

            if score > best_score {
                best_score = score;
                best_move = Some(scored_move.mv);
            }

            if score > alpha {
                alpha = score;
            }

            if alpha >= beta {
                cutoff = true;
                self.transposition_table.store(
                    hash_key,
                    depth,
                    best_score,
                    TtFlags::LOWER,
                    best_move
                );
                break;
            }

        }

        path.pop();

        if !cutoff  {
            self.transposition_table.store(
                hash_key,
                depth,
                best_score,
                if alpha > original_alpha { TtFlags::EXACT } else { TtFlags::UPPER },
                best_move
            );
        }
        return (best_move, best_score);
    }

    fn order_moves(&self, board: &chess::Board, move_: chess::Move, tt_move: Option<chess::Move>, phase: i32) -> i32 {
        let victim:Option<chess::Piece> = board.piece_on(move_.to);
        let attacker:Option<chess::Piece> = board.piece_on(move_.from);

        if tt_move.is_some(){
            if tt_move.unwrap() == move_ {
                return 1_000_000;
            }
        }

        if victim.is_none() {
            if let Some(ep_file) = board.en_passant() {
                // Calculate EP Rank to see if this move hits it
                let ep_rank = match board.side_to_move() {
                    chess::Color::White => chess::Rank::Sixth,
                    chess::Color::Black => chess::Rank::Third,
                };
                if move_.to == chess::Square::new(ep_file, ep_rank) {
                    if attacker == Some(chess::Piece::Pawn) {
                        return 900; // Correct Score for Pawn takes Pawn (100*10 - 100)
                    } 
                }
            }
            
        } else {

            let victim_value = ((MG_PIECE_VALUES[victim.unwrap() as usize] * phase) + (EG_PIECE_VALUES[victim.unwrap() as usize] * (MAX_PHASE - phase))) / MAX_PHASE;
            let attacker_value = ((MG_PIECE_VALUES[attacker.unwrap() as usize] * phase) + (EG_PIECE_VALUES[attacker.unwrap() as usize] * (MAX_PHASE - phase))) / MAX_PHASE;

            return (victim_value * 10) - attacker_value;
        }

        return 0;
    }

    fn is_repetition(&self, history: &Vec<u64>, path: &Vec<u64>, hash_key: u64) -> bool {
        let mut occurrences = 0;
        for past_hash in history {
            if past_hash == &hash_key {
                occurrences += 1;
                if occurrences >= 2 { return true; } // Early exit if found
            }
        }

        for past_hash in path {
            if past_hash == &hash_key {
                occurrences += 1;
                if occurrences >= 2 { return true; }
            }
        }
        return false;
    }

    fn evaluate_board(&mut self, board: &mut chess::Board, depth: i8, color: chess::Color, history: &Vec<u64>) -> i32 {
        let (_, score) = self.search(board, depth, color, i32::MIN, i32::MAX, history, &mut Vec::new());
        return score;
    }

    fn generate_move(&mut self, board: &mut chess::Board, depth: i8, color: chess::Color, history: &Vec<u64>) -> chess::Move {
        let (best_move, _) = self.search(board, depth, color, i32::MIN, i32::MAX, history, &mut Vec::new());
        return best_move.unwrap();
    }

    fn evaluate_and_move(&mut self, board: &mut chess::Board, max_depth: i8, color: chess::Color, history: &Vec<u64>) -> (chess::Move, i32) {
        let mut best_move: Option<chess::Move> = None;
        let mut score = 0;
        let delta = 25; // Aspiration window size (e.g., 0.25 pawns)

        let mut alpha = i32::MIN;
        let mut beta = i32::MAX;

        let mut path : Vec<u64> = Vec::new();

        for depth in 1..=max_depth {
            
            loop {
                (best_move, score) = self.search(board, depth, color, alpha, beta, &history, &mut path);
                
                if score <= alpha {
                    alpha = i32::MIN;
                    continue;
                } else if score >= beta {
                    beta = i32::MAX;
                    continue;
                } else {
                    alpha = score - delta;
                    beta = score + delta;
                    break;
                }
            }
            
        }

        // The result from the final iteration (max_depth)
        match best_move {
            Some(m) => return (m, score),
            Option::None => {
                // If we found no move, just grab the first legal move as a fallback
                // This prevents the crash so you can see what's wrong.
                let mut legal_moves = Vec::new();
                board.generate_moves(|moves| { legal_moves.extend(moves); false });
                
                if let Some(first_move) = legal_moves.first() {
                    eprintln!("Warning: Search returned None. Playing fallback move.");
                    return (*first_move, score);
                } else {
                    // No legal moves at all? It's Checkmate/Stalemate.
                    return (chess::Move{from: chess::Square::A1, to: chess::Square::A1, promotion: None}, score); // Or handle game over
                }
            }
        }
    }

}

fn main() ->Result<()> {

    let mut engine: Engine = Engine::new();
    let mut board: cozy_chess::Board;
    let mut buffer = String::new();
    loop {
        let _ = stdin().read_line(&mut buffer);
        buffer = buffer.trim().to_string();
        if buffer == "quit" {
            break Ok(());
        }

        let json: Request = serde_json::from_str(buffer.as_str())?;

        let command = json.command;
        let moves: Vec<String> = json.moves;
        let color: cozy_chess::Color = if json.color == "w" { cozy_chess::Color::White } else { cozy_chess::Color::Black };
        let depth: i8 = json.depth;
        let mut history: Vec<u64> = Vec::new();

        board = chess::Board::startpos();
        for mv_str in moves {
            let parsed_move = chess::util::parse_uci_move(&board, mv_str.as_str()).unwrap();
            board.play(parsed_move);
            history.push(board.hash());
        }

        if command == "eval" {


            let score = engine.evaluate_board(&mut board, depth, color, &history);
            println!("Score: {}", score);
            buffer.clear();
            continue;
        }

        else if command == "move" {
            let best_move = engine.generate_move(&mut board, depth, color, &history);
            println!("Best move: {}", best_move);
            board.play(best_move);
            buffer.clear();
        }

        else if command == "eval_move" {
            let (best_move, score) = engine.evaluate_and_move(&mut board, depth, color, &history);
            println!("{} {}", chess::util::display_san_move(&board, best_move), score);
            buffer.clear();
        }

    }

}