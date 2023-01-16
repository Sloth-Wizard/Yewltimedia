use gloo::timers::callback::Timeout;
use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use crate::models::cards::Card;

/// Build a `card` component with the referenced card data
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
        move |_: MouseEvent| {
            // Cast our cloned node as the correct type
            let sprite = button_play_sprite_ref.cast::<HtmlElement>();
            
            // Add the class and then remove it after 510 ms
            if let Some(sprite) = sprite {
                if !sprite.class_name().contains("sprite") {
                    play_sprite(sprite, animation_play_time);
                }
            }
        }
    };

    // Show More button logic
    let button_showmore_ref = use_node_ref();
    let event_showmore = {
        // Clone our node so we can use the clone without moving the original node
        let button_showmore_ref = button_showmore_ref.clone();
        move |_: MouseEvent| {
            // Cast our cloned node as the correct type
            let button: Option<HtmlElement> = button_showmore_ref.cast::<HtmlElement>();
            
            // Add or remove the class if our node exists
            if let Some(button) = button {
                if button.class_name().contains("tapped") {
                    button.set_class_name(button.class_name().replace(" tapped", "").as_str());
                } else {
                    button.set_class_name(format!("{} tapped", button.class_name()).as_str());
                }
            }
        }
    };

    html! {
        <>
            <card class={classes!(String::from("card"))}>
                <div ref={button_play_sprite_ref} onmousedown={event_play_sprite} class={classes!(String::from("card__img js-tap"))} data-k={lowercase_name.to_owned()}>
                    <img class={classes!(String::from("js-lazy"))} src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfQAAAD6CAMAAABgdUV8AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA3BpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDkuMC1jMDAwIDc5LjE3MWMyN2ZhYiwgMjAyMi8wOC8xNi0yMjozNTo0MSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpmZWIxMDU0OC02ZTlmLTRjMDEtOWRjYi1iMzc1NWNhZjMyZGUiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MTdFNjAzRTI4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MTdFNjAzRTE4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDI0LjAgKE1hY2ludG9zaCkiPiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDpCMEU5MDI5Qjc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDpCMEU5MDI5Qzc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PkggglcAAAAGUExURf///wAAAFXC034AAACTSURBVHja7MExAQAAAMKg9U9tDB+gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPiYAAMA6VEAAe661v0AAAAASUVORK5CYII=" data-src={format!("./public/{}", sprite)} alt={name.to_owned()} draggable="false"/>
                </div>
                <div class={classes!(String::from("card__actions"))}>
                    <div ref={button_showmore_ref} onmousedown={event_showmore} class={classes!(String::from("more js-tap"))} data-k={lowercase_name.to_owned()}>
                        <div class={classes!(String::from("icon"))}>
                            <span></span>
                            <span></span>
                        </div>
                    </div>
                </div>
            </card>
        </>
    }
}

#[wasm_bindgen]
extern "C" {
    // Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_err(s: &str);
}

// Add a class to the passed HtmlElement to move the sprite's position
#[wasm_bindgen]
pub fn play_sprite(el: HtmlElement, timeout: u32) {
    // Add the classes that animate the sprite
    el.set_class_name(format!("{} tapped sprite", el.class_name()).as_str());
                    
    // Remove the tapped class after anim play time and the sprite class after twice the anim play time
    Timeout::new(timeout, move || {
        el.set_class_name(el.class_name().replace(" tapped", "").as_str());
        Timeout::new(timeout, move || {
            el.set_class_name(el.class_name().replace(" sprite", "").as_str());
        }).forget();
    }).forget();
}
