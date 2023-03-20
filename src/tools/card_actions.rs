use gloo::timers::callback::Timeout;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, NodeList, window, Event};

use crate::tools::card_actions_bindings::*;

pub fn sprite_animation(sprite: Option<HtmlElement>, ms: u32) -> bool {
    if let Some(sprite) = sprite {
        if !sprite.class_name().contains("sprite") {
            // Add the classes that animate the sprite
            sprite.set_class_name(format!("{} tapped sprite", sprite.class_name()).as_str());
                            
            // Remove the tapped class after anim play time and the sprite class after twice the anim play time
            Timeout::new(ms, move || {
                sprite.set_class_name(sprite.class_name().replace(" tapped", "").as_str());
                Timeout::new(ms, move || {
                    sprite.set_class_name(sprite.class_name().replace(" sprite", "").as_str());
                }).forget();
            }).forget();
        }

        return true
    }

    false
}

pub struct CardElements {
    pub button: HtmlElement,
    pub icon: HtmlElement,
    pub card: HtmlElement,
    pub container: HtmlElement
}
pub enum OpenOrClose {
    // cards, button, icon, card
    Open(NodeList, CardElements),
    // cards, button, icon, card
    Close(NodeList, CardElements),
    // Card is still playing the animation so do nothing
    IsPlaying,
    // console.error("Noob")
    Error(&'static str)
}
pub fn show_more(button: Option<HtmlElement>, icon: Option<HtmlElement>, card: Option<HtmlElement>, desc_cont: Option<HtmlElement>) -> OpenOrClose {
    let mut card_elements: Vec<HtmlElement> = Vec::new();

    // Match our passed element, if any of the elements are missing, the function will end
    match button { Some(button) => card_elements.push(button), None => return OpenOrClose::Error("Missing button") }
    match icon { Some(icon) => card_elements.push(icon), None => return OpenOrClose::Error("Missing icon") }
    match card { Some(card) => card_elements.push(card), None => return OpenOrClose::Error("Missing card") }
    match desc_cont { Some(container) => card_elements.push(container), None => return OpenOrClose::Error("Missing description container") }

    if card_elements[0].class_name().contains("playing") {
        return OpenOrClose::IsPlaying;
    }

    // No missing elements so now we map it to a struct
    let card_mapping = CardElements {
        button: card_elements[0].clone(),
        icon: card_elements[1].clone(),
        card: card_elements[2].clone(),
        container: card_elements[3].clone()
    };

    // Gather all our cards
    let cards: NodeList = HtmlDocument::query_selector_all(&document, ".card");
    match card_mapping.button.get_attribute("data-nxtcl") {
        Some(nxtcl) => {
            if nxtcl == ">o_o>" {
                return OpenOrClose::Open(cards, card_mapping)
            } else {
                return OpenOrClose::Close(cards, card_mapping)
            }
        },
        None => {
            return OpenOrClose::Open(cards, card_mapping)
        }
    }
}

pub fn open_card(cards: NodeList, card: CardElements, ms: u32) -> Result<HtmlElement, JsValue> {
    card.button.set_class_name(format!("{} tapped playing", card.button.class_name()).as_str());
    match card.button.set_attribute("data-nxtcl", "<o_o<") {
        Ok(_) => (),
        Err(err) => return Err(err)
    };

    // Hide + sign of focused card and make sure other cards are z-index'd under our active card
    toggle_cards_visibility(cards, card.card, card.icon, 0, String::from("0"));

    // Remove playing class when animation is done
    let button = card.button.clone();
    Timeout::new(ms*2, move || {
        button.set_class_name(button.class_name().replace(" playing", "").as_str());
    }).forget();
    
    Ok(card.button)
}

pub fn close_card(cards: NodeList, card: CardElements, container: HtmlElement, ms: u32) -> Result<HtmlElement, JsValue> {
    card.button.set_class_name(format!("{} playing", card.button.class_name()).as_str());
    container.set_class_name(container.class_name().replace(" tapped", "").as_str());

    let button = card.button.clone();
    Timeout::new(ms, move || {
        button.set_class_name(button.class_name().replace(" tapped", "").as_str());
    
        // Reset + sign visibility and z-index positions
        toggle_cards_visibility(cards, card.card, card.icon, ms, String::from("auto"));

        // Remove playing class when animation is done
        Timeout::new(ms, move || {
            button.set_class_name(button.class_name().replace(" playing", "").as_str());
        }).forget();
    }).forget();

    match card.button.set_attribute("data-nxtcl", ">o_o>") {
        Ok(_) => (),
        Err(err) => return Err(err)
    };

    Ok(card.button)
}

#[derive(PartialEq)]
struct DescriptionElements {
    button: HtmlElement,
    container: HtmlElement
}
pub fn load_description(button: HtmlElement, container: HtmlElement, ms: u32) -> () {
    match lazyload() {
        Ok(ok) => {
            if !ok {
                error(JsValue::from_str("Could not create our custom event"))
            }
        },
        Err(err) => error(err)
    }

    Timeout::new(ms, move || {
        if button.class_name().contains("tapped") {
            container.set_class_name(format!("{} tapped", container.class_name()).as_str())
        }
    }).forget();
}

fn toggle_cards_visibility(cards: NodeList, card: HtmlElement, icon: HtmlElement, ms: u32, z_index: String) -> () {
    Timeout::new(ms, move || {
        for i in 0..cards.length() {
            if let Some(node) = cards.get(i) {
                if !card.is_same_node(Some(&node)) {
                    let el = node.dyn_into::<HtmlElement>().unwrap();
                    el.style().set_property("z-index", z_index.as_str()).unwrap()
                }
            }
        }

        icon.style().set_property("opacity", if z_index == "0" { "0" } else { "1" }).unwrap()
    }).forget();
}

// Send an event and trigger our lazyloading
fn lazyload() -> Result<bool, JsValue> {
    let window = window();
    let lazyload_event: Result<Event, JsValue> = Event::new("__lazyload");

    if let Some(window) = window {
        match lazyload_event {
            Ok(event) => return window.dispatch_event(&event),
            Err(err) => return Err(err)
        }
    }

    Ok(false)
}
