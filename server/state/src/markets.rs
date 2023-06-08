use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Market {
    pub id: u32,
    pub name: String,
    pub event: String,
}

pub fn get_markets() -> Vec<Market> {
    vec![
        Market { id: 1, name: "Mouz vs ENCE".to_string(), event: "BLAST.TV Major".to_string() },
        Market { id: 2, name: "G2 vs FaZe".to_string(), event: "BLAST.TV Major".to_string() },
        Market {
            id: 3,
            name: "Liquid vs Astralis".to_string(),
            event: "BLAST.TV Major".to_string(),
        },
    ]
}
