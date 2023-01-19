#[allow(non_upper_case_globals)] // To silence annoying warnings when using wasm static methods like document or body init's 

mod components;
mod data;
mod models;
mod tools;

use models::cards::*;
use yew::prelude::*;
use yew::{classes, html};
use data::cards_data::create_cards;

use crate::components::card::CardComponent;

#[function_component]
fn App() -> Html {
    let header_text = "en 2023,".to_string();
    let footer_text = "Multimédia\u{00a0}!".to_string();
    let description_container = use_node_ref();
    
    // Build a list of cards components
    let mut index = -1;
    let cards = create_cards().into_iter().map(|card| {
        index += 1;
        let props = CardComponentProps { card, description_container: description_container.clone() };
        html! {
            <>
                if index == 4 {
                    <div class={classes!(String::from("card card--mm"))}>
                        <div class={classes!(String::from("card__text card__text--mm"))}>
                            <p>{ "Voici" }</p>
                            <p>{ "l'équipe" }</p>
                        </div>
                    </div>
                    <CardComponent ..props.clone()/>
                } else {
                    <CardComponent ..props.clone()/>
                }
            </>
        }
    }).collect::<Html>();

    html! {
        <div class={classes!(String::from("container"))}>
            <div class={classes!(String::from("card--zone"))}>
                <div class={classes!(String::from("card--title card__text--mm"))}>
                    <p>{header_text}</p>
                </div>
                <cards class={classes!(String::from("card--wrapper"))}>
                    {cards}
                </cards>
                <div class="card--title card__text--mm">
                    <p>{footer_text}</p>
                </div>
            </div>
            <div ref={description_container} class="card--zone--desc _czd"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
