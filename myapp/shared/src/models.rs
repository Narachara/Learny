use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(tag = "type")] // “When you serialize this enum, inject a field called type whose value is the variant name.”
pub enum Block {
    Text { value: String },
    Math { value: String },
    Image { src: String },
    File { path: String },
}

impl Block {
    pub fn block_type(&self) -> &'static str {
        match self {
            Block::Text { .. } => "text",
            Block::Math { .. } => "math",
            Block::Image { .. } => "image",
            Block::File { .. } => "file",
        }
    }
}


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Card {
    pub id: i64,
    pub deck_id: i64,
    pub name: String,

    /// Blocks that belong to the question side (front)
    pub front_blocks: Vec<Block>,

    /// Blocks that belong to the answer side (back)
    pub back_blocks: Vec<Block>,

    pub created_at: i64,
    pub times_seen: u32,
    pub times_correct: u32,
    pub tags: Option<String>,
}

impl Card {
    pub fn new_empty(deck_id: i64) -> Self {
        Self {
            id: -1, // temporary ID; backend will assign real ID
            deck_id,
            name: "Neue Karte".into(),
            front_blocks: vec![],
            back_blocks: vec![],
            created_at: chrono::Utc::now().timestamp(),
            times_seen: 0,
            times_correct: 0,
            tags: None,
        }
    }

    pub fn progress_percent(&self) -> u8 {
        let good = self.times_correct as f64;
        let bad = (self.times_seen - self.times_correct) as f64;

        if good == 0.0 {
            return 0;
        }

        let alpha = 2.0; // BAD penalty
        let k = 0.6;     // curve speed

        let score = (good - alpha * bad).max(0.0);

        let confidence = 1.0 - (-k * score).exp();

        (confidence * 100.0).round().clamp(0.0, 100.0) as u8
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Deck {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub card_count: u32,
}