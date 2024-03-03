use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BestMove{
    pub best_move: String,
    pub score: f32
}