use api_utils::{json_list, json_parse_key_values};
use repress::{app as repress_app, request::Request, response::Response, router::RouterTrait};
use stockfish_adapter::StockfishAdapter;
mod api_utils;
mod stockfish_adapter;

pub fn main() {
    let mut app = repress_app();
    app.router.get(
        "/legalmoves",
        Box::new(move |req: &Request, res: &mut Response| {
            let mut stockfish = StockfishAdapter::new();
            println!("status: {}", stockfish.status());
            let fen = req.get_header("fen");
            let body = json_parse_key_values(req.body.as_str());
            let fen = body.get("fen").unwrap_or(&fen);
            println!("fen: {}", fen);
            stockfish.set_fen(fen.as_str());
            let legal_moves = stockfish.legal_moves();
            res.set_header("content-type", "text/json");
            res.set_status(200).text(json_list(legal_moves));
            res.end();
            stockfish.kill();
        }),
    );
    app.listen(4000, |port| println!("Serving on port {port}"));
}
