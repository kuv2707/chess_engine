use std::collections::HashMap;

use super::{models::BestMove, AppState};
use super::engine::{
    self,
    board::Position,
    move_as_string,
    moves::{all_possible_valid_moves, filter_out_check_moves, get_raw_moves},
    piece::Piece,
    Move,
};
use actix_web::{web, App, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Fen {
    fen: String,
}
pub async fn bestmove(fen: web::Json<Fen>) -> impl Responder {
    let board = engine::board::create_board(&fen.fen);
    if board.is_none() {
        return HttpResponse::BadRequest().body("Invalid FEN");
    }
    let mut board = board.unwrap();
    let (score, mov) = board.best_move(4);
    match mov {
        Some(best_move) => HttpResponse::Ok().json(BestMove {
            score,
            best_move: move_as_string(&best_move),
        }),
        None => HttpResponse::Ok().json(BestMove {
            score: 0.0,
            best_move: "".to_string(),
        }),
    }
}

#[derive(Deserialize)]
pub struct MoveArg {
    mov: String,
    fen: String,
}
pub async fn makemove(fen: web::Json<MoveArg>) -> impl Responder {
    let board = engine::board::create_board(&fen.fen);
    if board.is_none() {
        return HttpResponse::BadRequest().body("Invalid FEN");
    }
    let mut board = board.unwrap();
    let mov = &fen.mov;
    let m_res: Result<u16, String> = engine::parse_move(&mov);
    if m_res.is_err() {
        return HttpResponse::BadRequest().body("Invalid move");
    }
    let mov = m_res.unwrap();
    let valid_moves = all_possible_valid_moves(&mut board);
    if !valid_moves.contains(&mov) {
        return HttpResponse::BadRequest().body("Invalid move");
    }
    board.make_move(mov);
    HttpResponse::Ok().body(board.to_fen())
}

pub async fn piecewisemoves(fen: web::Json<Fen>) -> impl Responder {
    let board = engine::board::create_board(&fen.fen);
    if board.is_none() {
        return HttpResponse::BadRequest().body("Invalid FEN");
    }
    let mut board = board.unwrap();
    let mut mvs = HashMap::<Position, Vec<String>>::new();
    let piecemap = board.piecemap.clone();
    for (pos, piece) in piecemap {
        if piece.color != board.side_to_move {
            continue;
        }
        let rms = get_raw_moves(&piece, &pos, &board);
        let movs = filter_out_check_moves(&mut board, rms);
        mvs.insert(pos, movs.iter().map(|m| move_as_string(m)).collect());
    }
    HttpResponse::Ok().json(mvs)
}

pub async fn stockfishbestmove(data: web::Data<AppState>, fen: web::Json<Fen>) -> impl Responder {
    let mut stockfish = data.stockfish.lock().unwrap();
    stockfish.set_fen(&fen.fen);
    let bestmove = stockfish.bestmove();
    HttpResponse::Ok().body(bestmove)
}

//takes in a fen, finds best move for it and plays it, and returns updated fen
pub async fn stockfishmove(data: web::Data<AppState>, fen: web::Json<Fen>) -> impl Responder {
    let mut stockfish = data.stockfish.lock().unwrap();
    stockfish.set_fen(&fen.fen);
    let bestmove = stockfish.bestmove();
    let board = engine::board::create_board(&fen.fen);
    if board.is_none() {
        return HttpResponse::BadRequest().body("Invalid FEN");
    }
    let mut board = board.unwrap();
    let m_res: Result<u16, String> = engine::parse_move(&bestmove);
    if m_res.is_err() {
        return HttpResponse::BadRequest().body("Invalid move");
    }
    let mov = m_res.unwrap();
    board.make_move(mov);
    HttpResponse::Ok().body(board.to_fen())
}
