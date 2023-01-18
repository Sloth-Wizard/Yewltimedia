#[derive(Clone, PartialEq, yew::Properties)]
pub struct Card {
    pub picture: String,
    pub sprite: String,
    pub name: String,
    pub description: String,
    pub sound: String
}