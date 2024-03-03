use crate::engine::board::pos_as_string;

use self::board::Position;

pub mod board;
pub mod moves;
pub mod piece;
pub mod weights;


pub type Move= u16;


/*
@returns a tuple of (from,to)    
 */
pub fn decode_move(m: &Move) -> (Position, Position) {
    let from = (m >> 6) as u8;
    let to = (m & 0b111111) as u8;
    (from, to)
}
pub fn encode_move(from: Position, to: Position) -> Move {
    ((from as u16) << 6) | (to as u16)
}

pub fn move_as_string(m: &Move) -> String {
    let (from, to) = decode_move(m);
    format!("{}{}", pos_as_string(&from), pos_as_string(&to))
}


pub fn parse_move(m: &str) -> Result<Move, String> {
    if m.len() != 4 {
        return Err("invalid move string".to_string());
    }
    let from = parse_pos(&m[0..2])?;
    let to = parse_pos(&m[2..4])?;
    if from >= 64 || to >= 64 {
        return Err("invalid move".to_string());
    }
    Ok(encode_move(from, to))
}

pub fn parse_pos(m: &str) -> Result<Position, String> {
    let f=m.chars().nth(0).unwrap();
    let r=m.chars().nth(1).unwrap();
    if f < 'a' || f > 'h' || r < '1' || r > '8' {
        return Err("invalid position".to_string());
    }
    Ok(board::encode_pos(7-(r as u8 - '1' as u8), f as u8 - 'a' as u8))
}