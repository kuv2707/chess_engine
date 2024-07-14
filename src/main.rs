use api_utils::{json_list, json_parse_key_values};
use repress::{
    app as repress_app, request::Request, response::Response, router::RouterTrait, types::NextFn,
};
use stockfish_adapter::StockfishAdapter;
mod api_utils;
mod stockfish_adapter;

pub fn main() {
    let mut app = repress_app();
    app.middleware(Box::new(
        move |req: &Request, res: &mut Response, mut next: Box<NextFn>| {
            // add CORS headers
            res.set_header("Access-Control-Allow-Origin", "*");
            res.set_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS");
            res.set_header("Access-Control-Allow-Headers", "Content-Type");
            res.set_header("content-type", "application/json");
            println!("{} {:?}", req.verb, req.path);
            next();
        },
    ));
    app.router.get(
        "/stockfish",
        Box::new(move |req: &Request, res: &mut Response| {
            let body = json_parse_key_values(req.body.as_str());
            let fen = body.get("fen");
            if fen.is_none() {
                res.set_status(400)
                    .text("{\"error\": \"missing fen\"}".to_string());
                res.end();
                return;
            }
            let fen = fen.unwrap();
            let mut stockfish = StockfishAdapter::new();
            if (!stockfish.status()) {
                res.set_status(500)
                    .text("{\"error\": \"stockfish not running\"}".to_string());
                res.end();
                return;
            }
            println!("fen: {}", fen);
            stockfish.set_fen(fen.as_str());
            let op = body.get("operation");
            match op {
                Some(op) => {
                    if op.eq("bestmove") {
                        let bestmove = stockfish.bestmove();
                        res.set_status(200)
                            .text(format!("{{\"bestmove\": \"{}\"}}", bestmove));
                    } else if (op.eq("legalmoves")) {
                        let legal_moves = stockfish.legal_moves();
                        res.set_status(200).text(json_list(legal_moves));
                    } else {
                        res.set_status(400)
                            .text("{\"error\": \"invalid operation\"}".to_string());
                    }
                    res.end();
                    stockfish.kill();
                    return;
                }
                None => {}
            }
            res.end();
            stockfish.kill();
        }),
    );
    app.listen(4000, |port| println!("Serving on port {port}"));
}
