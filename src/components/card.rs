use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::models::cards::Card;

/// Build a `card` component with the referenced card data
#[function_component]
pub fn CardComponent(card: &Card) -> Html {
    let Card { picture: _, sprite, name, description: _, sound: _ } = card;
    let lowercase_name = name.to_lowercase();

    let onclick = {
        move |ev: MouseEvent| {
            log(&ev);
        }
    };

    html! {
        <>
            <card class={classes!(String::from("card"))}>
                <div class={classes!(String::from("card__img js-tap"))} data-k={lowercase_name.to_owned()}>
                    <img class={classes!(String::from("js-lazy"))} src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfQAAAD6CAMAAABgdUV8AAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJlYWR5ccllPAAAA3BpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1wQ2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFkb2JlIFhNUCBDb3JlIDkuMC1jMDAwIDc5LjE3MWMyN2ZhYiwgMjAyMi8wOC8xNi0yMjozNTo0MSAgICAgICAgIj4gPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNjcmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wTU09Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9tbS8iIHhtbG5zOnN0UmVmPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VSZWYjIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDpmZWIxMDU0OC02ZTlmLTRjMDEtOWRjYi1iMzc1NWNhZjMyZGUiIHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MTdFNjAzRTI4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MTdFNjAzRTE4Q0UxMTFFREEwRkRDMzUyQUI0QjUzRDEiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIDI0LjAgKE1hY2ludG9zaCkiPiA8eG1wTU06RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDpCMEU5MDI5Qjc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDpCMEU5MDI5Qzc4NzkxMUVEOTg3NkU3M0RDQTVCQzlENCIvPiA8L3JkZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PkggglcAAAAGUExURf///wAAAFXC034AAACTSURBVHja7MExAQAAAMKg9U9tDB+gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAPiYAAMA6VEAAe661v0AAAAASUVORK5CYII=" data-src={format!("./public/{}", sprite)} alt={name.to_owned()} draggable="false"/>
                </div>
                <div class={classes!(String::from("card__actions"))}>
                    <div {onclick} class={classes!(String::from("more js-tap"))} data-k={lowercase_name.to_owned()}>
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
    fn log(s: &MouseEvent);
}
