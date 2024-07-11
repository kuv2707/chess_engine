use api_utils::json_list;
use repress::{app as repress_app, request::Request, response::Response, router::RouterTrait};
use stockfish_adapter::StockfishAdapter;
mod api_utils;
mod stockfish_adapter;

pub fn main() {
    let mut app = repress_app();
    app.router.get(
        "/legalmoves",
        Box::new(move |req: &Request, res: &mut Response| {
            res.set_header("content-type", "text/json");
            let mut stockfish = StockfishAdapter::new();
            println!("status: {}", stockfish.status());
            let fen = req.get_header("fen");
            stockfish.set_fen(fen.as_str());
            let legal_moves = stockfish.legal_moves();
            res.set_status(200).text(json_list(legal_moves));
            res.end();
        }),
    );
    app.listen(4000, |port| println!("Serving on port {port}"));
}
