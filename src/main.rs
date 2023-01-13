mod components;
mod data;
mod models;

use yew::prelude::*;
use yew::{classes, html};
use data::cards_data::create_cards;
use components::card::CardComponent;

#[function_component]
fn App() -> Html {
    let header_text = "en 2023,".to_string();
    let footer_text = "Multimédia\u{00a0}!".to_string();

    html! {
        <div class={classes!(String::from("container"))}>
            <div class={classes!(String::from("card--zone"))}>
                <div class={classes!(String::from("card--title card__text--mm"))}>
                    <p>{header_text}</p>
                </div>
                <cards class={classes!(String::from("card--wrapper"))}>
                    {
                        create_cards().into_iter().map(|card| {
                            html! { <CardComponent ..card.clone()/> }
                        }).collect::<Html>()
                    }
                </cards>
                <div class="card--title card__text--mm">
                    <p>{footer_text}</p>
                </div>
            </div>
            <div class="card--zone--desc _czd"></div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
