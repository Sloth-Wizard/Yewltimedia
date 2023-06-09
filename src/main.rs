#[allow(non_upper_case_globals)] // To silence annoying warnings when using wasm static methods like document or body init's 

mod components;
mod data;
mod models;
mod tools;

use models::cards::*;
use models::imageloader::*;
use web_sys::HtmlElement;
use yew::prelude::*;
use yew::{classes, html};
use data::cards_data::create_cards;

use crate::components::imageloader::ImageLoaderComponent;
use crate::components::card::CardComponent;
use crate::tools::card_actions_bindings::*;
use crate::tools::constants::PRELOAD_DESC_IMAGE;

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

    let onclick = {
        let description_container = description_container.clone();
        move |_| {
            let container = description_container.cast::<HtmlElement>();
            if let Some(_) = container {
                let active_card = HtmlDocument::query_selector(&document, ".more.tapped");
                if let Some(card) = active_card {
                    let event = Event::new("mousedown");
                    match event {
                        Ok(event) => match card.dispatch_event(&event) {
                            Ok(ok) => ok,
                            Err(err) => {
                                error(err.clone());
                                false
                            }
                        },
                        Err(err) => {
                            error(err.clone());
                            false
                        }
                    };
                }
            };
        }
    };

    let img_props = ImageProps { src: String::from(PRELOAD_DESC_IMAGE), rs_src: String::from("http://127.0.0.1:8080/yewltmedia/public/Ali.png"), alt: String::from("Ali") };

    html! {
        <>
            <ImageLoaderComponent ..img_props.clone() />
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
                <div ref={description_container} {onclick} class="card--zone--desc _czd"></div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
