use yew::prelude::*;
use yew::{Component, Context};
use web_sys::HtmlElement;

use wasm_bindgen::prelude::*;

use crate::models::cards::Card;
use crate::models::fetch_states::FetchState;
use crate::tools::card_actions::*;
use crate::tools::fetchers::fetch_html;

const HTML_URL: &str = "http://127.0.0.1:8080/public/cardDescription.html";
const ANIM_MS: u32 = 0_510;

// Enumerate all the methods that should be available to the component here
pub enum Msg {
    PlaySprite(NodeRef),
    //OpenCard,
    //CloseCard,
    SetKDataFetchState(FetchState<String>),
    GetKData,
    //GetError
}

pub struct CardComponent {
    card_data: Card,
    k_data: FetchState<String>,
    sprite_ref: NodeRef
}

impl Component for CardComponent {
    type Message = Msg;
    type Properties = Card;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            card_data: ctx.props().to_owned(),
            k_data: FetchState::NotFetching,
            sprite_ref: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Card actions
            Msg::PlaySprite(sprite) => { sprite_animation(sprite.cast::<HtmlElement>(), ANIM_MS) }

            // kData fetcher
            Msg::SetKDataFetchState(fetch_state) => {
                self.k_data = fetch_state;
                true
            }
            Msg::GetKData => {
                ctx.link().send_future(async {
                    match fetch_html(HTML_URL).await {
                        Ok(k_data) => Msg::SetKDataFetchState(FetchState::Success(k_data)),
                        Err(err) => Msg::SetKDataFetchState(FetchState::Failed(err))
                    }
                });
                ctx.link().send_message(Msg::SetKDataFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &self.k_data {
            FetchState::NotFetching => {
                let sprite = self.sprite_ref.clone();
                let card_data = self.card_data.clone();
                html! {
                    <>
                        <card class={classes!(String::from("card"))}>
                            <div ref={self.sprite_ref.clone()} onmousedown={ctx.link().callback(move |_| Msg::PlaySprite(sprite.clone()))} class={classes!(String::from("card__img js-tap"))} data-k={card_data.name.to_lowercase()}>
                                <img class={classes!(String::from("js-lazy"))} src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfQAAAD6CAMAAABgdUV8AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA3BpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDkuMC1jMDAwIDc5LjE3MWMyN2ZhYiwgMjAyMi8wOC8xNi0yMjozNTo0MSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpmZWIxMDU0OC02ZTlmLTRjMDEtOWRjYi1iMzc1NWNhZjMyZGUiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MTdFNjAzRTI4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MTdFNjAzRTE4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDI0LjAgKE1hY2ludG9zaCkiPiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDpCMEU5MDI5Qjc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDpCMEU5MDI5Qzc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PkggglcAAAAGUExURf///wAAAFXC034AAACTSURBVHja7MExAQAAAMKg9U9tDB+gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPiYAAMA6VEAAe661v0AAAAASUVORK5CYII=" data-src={format!("./public/{}", card_data.sprite)} alt={card_data.name.to_owned()} draggable="false"/>
                            </div>
                            <div class={classes!(String::from("card__actions"))}>
                                <div class={classes!(String::from("more js-tap"))} data-k={card_data.name.to_lowercase()}>
                                    <div class={classes!(String::from("icon"))}>
                                        <span></span>
                                        <span></span>
                                    </div>
                                </div>
                            </div>
                        </card>
                    </>
                }
            },
            FetchState::Fetching => html! {<></>},
            FetchState::Success(data) => html! {
                <>
                    <card-description>
                        {data}
                    </card-description>
                </>
            },
            FetchState::Failed(err) => html! { err }
        }
    }
}

#[wasm_bindgen]
extern "C" {
    // Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: Option<HtmlElement>);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str, c: &str, d: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_err(s: &str);
}

/*
#[function_component]
pub fn CardComponent(card: &Card) -> Html {
    let Card { picture: _, sprite, name, description: _, sound: _ } = card;
    let lowercase_name = name.to_lowercase();
    let animation_play_time = 0_510; // Our animation play time in milliseconds

    // Sprite animation logic
    let button_play_sprite_ref = use_node_ref();
    let event_play_sprite = {
        // Clone our node
        let button_play_sprite_ref = button_play_sprite_ref.clone();
        move |_| {
            // Cast our cloned node as the correct type
            let sprite = button_play_sprite_ref.cast::<HtmlElement>();
            
            // Add the class and then remove it after 510 ms
            if let Some(sprite) = sprite {
                if !sprite.class_name().contains("sprite") {
                    // Add the classes that animate the sprite
                    sprite.set_class_name(format!("{} tapped sprite", sprite.class_name()).as_str());
                                    
                    // Remove the tapped class after anim play time and the sprite class after twice the anim play time
                    Timeout::new(animation_play_time, move || {
                        sprite.set_class_name(sprite.class_name().replace(" tapped", "").as_str());
                        Timeout::new(animation_play_time, move || {
                            sprite.set_class_name(sprite.class_name().replace(" sprite", "").as_str());
                        }).forget();
                    }).forget();
                }
            }
        }
    };

    // Show More button logic
    let button_showmore_ref = use_node_ref();
    let button_showmore_icon_ref = use_node_ref();
    let button_showmore_active_card_ref = use_node_ref();
    let event_showmore = {
        // Clone our node so we can use the clone without moving the original node
        let button_showmore_ref = button_showmore_ref.clone();
        let button_showmore_icon_ref = button_showmore_icon_ref.clone();
        let button_showmore_active_card_ref = button_showmore_active_card_ref.clone();

        // Handle our event
        move |_| {
            // Cast our cloned node as the correct type
            let button = button_showmore_ref.cast::<HtmlElement>();
            let icon = button_showmore_icon_ref.cast::<HtmlElement>();
            let active_card = button_showmore_active_card_ref.cast::<HtmlElement>();

            // Add or remove the class if our node exists
            if let Some(button) = button {
                // Get all the cards
                let cards = HTMLDocument::query_selector_all(&document, ".card");
                let description_container = HTMLDocument::query_selector(&document, "._czd");

                // Check if the needed elements exist
                if let Some(active_card) = active_card {
                    if let Some(icon) = icon {
                        match button.get_attribute("data-nxtcl") {
                            Some(data) => {
                                if data == ">o_o>" {
                                    match open_card(&button, active_card, cards, icon, &description_container) {
                                        Ok(_) => (),
                                        Err(err) => log_err(format!("{:?}", err.as_string()).as_str())
                                    };
                                } else if data == "<o_o<" {
                                    close_card(&button, active_card, cards, icon, animation_play_time).unwrap();
                                }
                            },
                            None => {
                                match open_card(&button, active_card, cards, icon, &description_container) {
                                    Ok(_) => (),
                                    Err(err) => log_err(format!("{:?}", err.as_string()).as_str())
                                }
                            }
                        };
                    }
                }
            }
        }

    };

    html! {
        <>
            <card ref={button_showmore_active_card_ref} class={classes!(String::from("card"))}>
                <div ref={button_play_sprite_ref} onmousedown={event_play_sprite} class={classes!(String::from("card__img js-tap"))} data-k={lowercase_name.to_owned()}>
                    <img class={classes!(String::from("js-lazy"))} src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfQAAAD6CAMAAABgdUV8AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA3BpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDkuMC1jMDAwIDc5LjE3MWMyN2ZhYiwgMjAyMi8wOC8xNi0yMjozNTo0MSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpmZWIxMDU0OC02ZTlmLTRjMDEtOWRjYi1iMzc1NWNhZjMyZGUiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MTdFNjAzRTI4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MTdFNjAzRTE4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDI0LjAgKE1hY2ludG9zaCkiPiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDpCMEU5MDI5Qjc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDpCMEU5MDI5Qzc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PkggglcAAAAGUExURf///wAAAFXC034AAACTSURBVHja7MExAQAAAMKg9U9tDB+gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPiYAAMA6VEAAe661v0AAAAASUVORK5CYII=" data-src={format!("./public/{}", sprite)} alt={name.to_owned()} draggable="false"/>
                </div>
                <div class={classes!(String::from("card__actions"))}>
                    <div ref={button_showmore_ref} onmousedown={event_showmore} class={classes!(String::from("more js-tap"))} data-k={lowercase_name.to_owned()}>
                        <div ref={button_showmore_icon_ref} class={classes!(String::from("icon"))}>
                            <span></span>
                            <span></span>
                        </div>
                    </div>
                </div>
            </card>
        </>
    }
}

///
/// WASM bindings for the component
/// 
#[wasm_bindgen]
extern "C" {
    // Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_err(s: &str);
}

#[wasm_bindgen]
extern {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method, js_name = querySelector)]
    fn query_selector(this: &HTMLDocument, selector: &str) -> HtmlElement;
    #[wasm_bindgen(method, js_name = querySelectorAll)]
    fn query_selector_all(this: &HTMLDocument, selector: &str) -> NodeList;
}
*/