use crate::models::cards::Card;

pub fn create_cards() -> Vec<Card> {
    return vec![
        Card {
            picture: "Alexis-mini.png".to_string(),
            sprite: "Alexis.png".to_string(),
            name: "Alexis".to_string(),
            description: "<p><i>\"In Rust We Trust 🦀\"</i></p><strong>Tu fais quoi de tes journées&nbsp;?</strong><p>Je tapotte des trucs au clavier et ca fait des chocapics</p><strong>Une passion&nbsp;?</strong><p>Le DIY</p><strong>À la maison il y a&nbsp;?</strong><p>Moi</p><strong>Un pseudo&nbsp;?</strong><p>└[∵┌]└[ ∵ ]┘[┐∵]┘</p><strong>Un dicton/mantra&nbsp;?</strong><p>Y a moyen de gagner encore un octet</p><strong>Un film préféré&nbsp;?</strong><p><a href=\"https://www.youtube.com/watch?v=JJkPLYmUyzg\" target=\"_blank\">Enter the Void - Gaspar Noé</a></p><strong>Une musique préférée&nbsp;?</strong><p><a href=\"https://www.youtube.com/watch?v=dQw4w9WgXcQ\" target=\"_blank\">All is Violent, All is Bright - God Is An Astronaut</a></p><strong>Ta BD préférée&nbsp;?</strong><p>La Guerre éternelle</p><strong>Ce que tu aimes chez Dupuis&nbsp;?</strong><p>Qu'on se sente comme à la maison</p><strong>Quelle est ta bonne résolution pour 2023&nbsp;?</strong><p>REDACTED</p>".to_string(),
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
