use crate::engine::board::pos_as_string;

use super::{
    board::{decode_pos, encode_pos, print_moves, Board, Position},
    decode_move, encode_move,
    piece::{Piece, PieceColor, PieceType},
    Move,
};

//define macro to get nth bit as i8

//raw move means those moves are not excluded which can lead to the same side getting a check

macro_rules! nth_bit {
    ($n:expr, $bit:expr) => {
        (((($n) >> ($bit)) & 1) as i8)
    };
}

macro_rules! in_bounds {
    ($r:expr, $f:expr) => {
        ($r < 8 && $f < 8 && $r >= 0 && $f >= 0)
    };
}

fn position_occupied(board: &Board, pos: &Position) -> bool {
    let piece = board.get_piece(*pos);
    return piece.is_some();
}

fn is_opponent_piece_at(board: &Board, pos: &Position) -> bool {
    let piece = board.get_piece(*pos);
    if piece.is_some() {
        return piece.unwrap().color != board.side_to_move;
    }
    return false;
}

pub fn slant_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(&base);
    let mut dir = 0;
    while dir <= 3 {
        let coeff_1 = nth_bit!(dir, 0) * 2 - 1;
        let coeff_2 = nth_bit!(dir, 1) * 2 - 1;
        let mut i = 1;
        loop {
            let newx: i16 = (r as i8 + i * coeff_1) as i16;
            let newy: i16 = (f as i8 + i * coeff_2) as i16;
            if !in_bounds!(newx, newy) {
                break;
            }
            let m = encode_pos(newx as u8, newy as u8);
            if position_occupied(board, &m) {
                if is_opponent_piece_at(board, &m) {
                    moves.push(m);
                }
                break;
            }
            moves.push(m);
            i += 1;
        }
        dir += 1;
    }

    moves
}

pub fn rect_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(&base);
    let coeffs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut dir: usize = 0;
    while dir <= 3 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let mut i = 1;
        loop {
            let newx: i16 = r as i16 + i * coeff_1;
            let newy: i16 = f as i16 + i * coeff_2;
            // println!("{} {}", newx, newy);
            if !in_bounds!(newx, newy) {
                break;
            }
            let m = encode_pos(newx as u8, newy as u8);
            if position_occupied(board, &m) {
                if is_opponent_piece_at(board, &m) {
                    moves.push(m);
                }
                break;
            }
            moves.push(m);

            i += 1;
        }
        dir += 1;
    }

    moves
}

pub fn knight_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(&base);
    let coeffs = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];
    let mut dir: usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_none() || is_opponent_piece_at(board, &m) {
                moves.push(m);
            }
        }
        dir += 1;
    }

    moves
}

pub fn king_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(&base);
    let coeffs = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut dir: usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_none() || is_opponent_piece_at(board, &m) {
                moves.push(m);
            }
        }
        dir += 1;
    }

    moves
}

pub fn pawn_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let color = board.side_to_move;
    let mut moves: Vec<Position> = Vec::new();
    let (r, f) = decode_pos(&base);
    let coeff = match color {
        super::piece::PieceColor::WHITE => -1,
        super::piece::PieceColor::BLACK => 1,
    };
    let newx: i16 = r as i16 + coeff;
    let newy: i16 = f as i16;
    if in_bounds!(newx, newy) {
        let m = encode_pos(newx as u8, newy as u8);
        let p = board.get_piece(m);
        if p.is_none() {
            moves.push(m);
        }
    }
    if (r == 6 && color == super::piece::PieceColor::WHITE)
        || (r == 1 && color == super::piece::PieceColor::BLACK)
    {
        let newx: i16 = r as i16 + 2 * coeff;
        let newy: i16 = f as i16;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_none() {
                moves.push(m);
            }
        }
    }
    let coeffs = [(1, 1), (1, -1)];
    for cof in coeffs {
        let coeff_1 = cof.0;
        let coeff_2 = cof.1;
        let newx: i16 = r as i16 + coeff_1 * coeff;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            if is_opponent_piece_at(board, &m) {
                moves.push(m);
            }
        }
    }

    moves
}

pub fn rook_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    rect_moves_raw(base, board)
}
pub fn bishop_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    slant_moves_raw(base, board)
}
pub fn queen_moves_raw(base: Position, board: &Board) -> Vec<Position> {
    let mut moves: Vec<Position> = Vec::new();
    moves.append(&mut slant_moves_raw(base, &board));
    moves.append(&mut rect_moves_raw(base, &board));
    moves
}

pub fn get_raw_moves(p: &Piece, pos: &Position, board: &Board) -> Vec<Move> {
    let srcvec = match p.piece_type {
        PieceType::PAWN => pawn_moves_raw(*pos, board),
        PieceType::BISHOP => bishop_moves_raw(*pos, board),
        PieceType::KING => king_moves_raw(*pos, board),
        PieceType::KNIGHT => knight_moves_raw(*pos, board),
        PieceType::QUEEN => queen_moves_raw(*pos, board),
        PieceType::ROOK => rook_moves_raw(*pos, board),
    };
    srcvec.iter().map(|dest| encode_move(*pos, *dest)).collect()
}

pub fn filter_out_check_moves(board: &mut Board, raw_moves: Vec<Move>) -> Vec<Move> {
    let stm = board.side_to_move;
    let mut valid_moves: Vec<Move> = Vec::new();
    for m in raw_moves {
        let ctx = board.make_move(m);
        if !board.has_check(&stm) {
            valid_moves.push(m);
        }
        board.unmake_move(ctx);
    }
    valid_moves
}

pub fn all_possible_raw_moves(board: &Board) -> Vec<Move> {
    // checkout whose turn it is from board
    // filter out all pieces of that color from board.squares
    // for each piece, get its raw moves
    // filter moves which cause same side to get a check
    // return vector of Move's
    let mut raw_moves: Vec<Move> = Vec::new();
    //sort it such that queen moves are first
    for (loc, piece) in &board.piecemap {
        if piece.color == board.side_to_move {
            let mut rm = get_raw_moves(&piece, &loc, &board);
            // println!("{:?} {:?}",piece.piece_type ,rm.iter().map(|m| decode_move(&m).1).map(|n| decode_pos(&n)).collect::<Vec<_>>());
            raw_moves.append(&mut rm);
        }
    }
    raw_moves
}

pub fn all_possible_valid_moves(board: &mut Board) -> Vec<Move> {
    filter_out_check_moves(board, all_possible_raw_moves(board))
}

pub fn find_in_raw_move_targets(
    board: &Board,
    searchpos: &Position,
    opponent_col: &PieceColor,
) -> bool {
    square_search(board, searchpos, opponent_col)
        || diag_search(board, searchpos, opponent_col)
        || knight_search(board, searchpos, opponent_col)
        || pawn_search(board, searchpos, opponent_col)
        || king_search(board, searchpos, opponent_col)
}

pub fn square_search(board: &Board, searchpos: &Position, opponent_col: &PieceColor) -> bool {
    let (r, f) = decode_pos(searchpos);
    let mut i = r + 1;
    while i < 8 {
        let p = board.get_piece(encode_pos(i as u8, f as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::ROOK)
            {
                return true;
            } else {
                break;
            }
        }
        i += 1;
    }
    i = r - 1;
    while i >= 0 {
        let p = board.get_piece(encode_pos(i as u8, f as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::ROOK)
            {
                return true;
            } else {
                break;
            }
        }
        i -= 1;
    }
    i = f + 1;
    while i < 8 {
        let p = board.get_piece(encode_pos(r as u8, i as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::ROOK)
            {
                return true;
            } else {
                break;
            }
        }
        i += 1;
    }
    i = f - 1;
    while i >= 0 {
        let p = board.get_piece(encode_pos(r as u8, i as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::ROOK)
            {
                return true;
            } else {
                break;
            }
        }
        i -= 1;
    }

    return false;
}

pub fn diag_search(board: &Board, searchpos: &Position, opponent_col: &PieceColor) -> bool {
    let (r, f) = decode_pos(searchpos);
    let mut i = r + 1;
    let mut j = f + 1;
    while i < 8 && j < 8 {
        let p = board.get_piece(encode_pos(i as u8, j as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::BISHOP)
            {
                return true;
            } else {
                break;
            }
        }
        i += 1;
        j += 1;
    }
    i = r - 1;
    j = f - 1;
    while i >= 0 && j >= 0 {
        let p = board.get_piece(encode_pos(i as u8, j as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::BISHOP)
            {
                return true;
            } else {
                break;
            }
        }
        i -= 1;
        j -= 1;
    }
    i = r + 1;
    j = f - 1;
    while i < 8 && j >= 0 {
        let p = board.get_piece(encode_pos(i as u8, j as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::BISHOP)
            {
                return true;
            } else {
                break;
            }
        }
        i += 1;
        j -= 1;
    }
    i = r - 1;
    j = f + 1;
    while i >= 0 && j < 8 {
        let p = board.get_piece(encode_pos(i as u8, j as u8));
        if p.is_some() {
            if p.unwrap().color == *opponent_col
                && (p.unwrap().piece_type == PieceType::QUEEN
                    || p.unwrap().piece_type == PieceType::BISHOP)
            {
                return true;
            } else {
                break;
            }
        }
        i -= 1;
        j += 1;
    }
    return false;
}

pub fn knight_search(board: &Board, searchpos: &Position, opponent_col: &PieceColor) -> bool {
    let (r, f) = decode_pos(searchpos);
    let coeffs = [
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ];
    let mut dir: usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_some()
                && p.unwrap().color == *opponent_col
                && p.unwrap().piece_type == PieceType::KNIGHT
            {
                return true;
            }
        }
        dir += 1;
    }

    return false;
}

pub fn king_search(board: &Board, searchpos: &Position, opponent_col: &PieceColor) -> bool {
    let (r, f) = decode_pos(searchpos);
    let coeffs = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let mut dir: usize = 0;
    while dir <= 7 {
        let coeff_1 = coeffs[dir].0;
        let coeff_2 = coeffs[dir].1;
        let newx: i16 = r as i16 + coeff_1;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_some()
                && p.unwrap().color == *opponent_col
                && p.unwrap().piece_type == PieceType::KING
            {
                return true;
            }
        }
        dir += 1;
    }

    return false;
}

pub fn pawn_search(board: &Board, searchpos: &Position, opponent_col: &PieceColor) -> bool {
    let (r, f) = decode_pos(searchpos);
    let coeff = match opponent_col {
        super::piece::PieceColor::WHITE => 1,
        super::piece::PieceColor::BLACK => -1,
    };
    let coeffs = [(1, 1), (1, -1)];
    for cof in coeffs {
        let coeff_1 = cof.0;
        let coeff_2 = cof.1;
        let newx: i16 = r as i16 + coeff_1 * coeff;
        let newy: i16 = f as i16 + coeff_2;
        if in_bounds!(newx, newy) {
            let m = encode_pos(newx as u8, newy as u8);
            let p = board.get_piece(m);
            if p.is_some()
                && p.unwrap().color == *opponent_col
                && p.unwrap().piece_type == PieceType::PAWN
            {
                let p = board.get_piece(m);
                if p.is_some() && p.unwrap().piece_type == PieceType::PAWN {
                    return true;
                }
            }
        }
    }

    return false;
}
