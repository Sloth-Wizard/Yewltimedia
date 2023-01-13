use crate::models::cards::Card;

pub fn create_cards() -> Vec<Card> {
    return vec![
        Card {
            picture: "Alexis-mini.png".to_string(),
            sprite: "Alexis.png".to_string(),
            name: "Alexis".to_string(),
            description: "<p><i>\"In Rust We Trust 🦀\"</i></p>".to_string(),
            sound: "tRex.mp3".to_string()
        },
        Card {
            picture: "Marc-mini.png".to_string(),
            sprite: "Marc.png".to_string(),
            name: "Marc".to_string(),
            description: "<p>Travaille actuellement sous couverture&nbsp;!</p>".to_string(),
            sound: "marioJump.mp3".to_string()
        },
    ];
}
