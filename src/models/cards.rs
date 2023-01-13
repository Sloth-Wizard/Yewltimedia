#[derive(Clone, PartialEq, yew::Properties)]
pub struct Card {
    pub(crate) picture: String,
    pub(crate) sprite: String,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) sound: String
}