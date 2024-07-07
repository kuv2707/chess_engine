use repress::{app as repress_app, request::Request, response::Response, router::RouterTrait};
use stockfish_adapter::StockfishAdapter;
mod stockfish_adapter;

pub fn main() {
    let mut app = repress_app();
    let stockfish = StockfishAdapter::new();
    app.router.get(
        "/",
        Box::new(move |req: &Request, res: &mut Response| {
            res.set_header("content-type", "text/html");

            res.text(req.body.clone());
            println!("{}", stockfish.pid());
            res.set_status(200).end();
        }),
    );
    app.listen(4000, |port| println!("Serving on port {port}"));
}
