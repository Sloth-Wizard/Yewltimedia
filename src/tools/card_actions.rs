use gloo::timers::callback::Timeout;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{HtmlElement, NodeList};

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

#[wasm_bindgen]
pub fn open_card(button: &HtmlElement, active_card: HtmlElement, cards: NodeList, icon: HtmlElement, description_container: &HtmlElement) -> Result<(), JsValue> {
    button.set_class_name(format!("{} tapped", button.class_name()).as_str());
    match button.set_attribute("data-nxtcl", "<o_o<") {
        Ok(_) => (),
        Err(err) => return Err(err)
    };

    // Hide + signs of unfocused cards
    for i in 0..cards.length() {
        if let Some(card) = cards.get(i) {
            if !active_card.is_same_node(Some(&card)) {
                let el = card.dyn_into::<HtmlElement>()?;
                el.style().set_property("z-index", "0")?;
            }
        }
    }

    icon.style().set_property("opacity", "0")?;

    Ok(())
}

#[wasm_bindgen]
pub fn close_card(button: &HtmlElement, active_card: HtmlElement, cards: NodeList, icon: HtmlElement, timeout: u32) -> Result<(), JsValue> {
    button.set_class_name(button.class_name().replace(" tapped", "").as_str());
    match button.set_attribute("data-nxtcl", ">o_o>") {
        Ok(_) => (),
        Err(err) => return Err(err)
    };

    // Display + signs of unfocused cards
    // We wait the main animation to finish before changing back ou z-index
    Timeout::new(timeout, move || {
        for i in 0..cards.length() {
            if let Some(card) = cards.get(i) {
                if !active_card.is_same_node(Some(&card)) {
                    let el = card.dyn_into::<HtmlElement>().unwrap();
                    el.style().set_property("z-index", "auto").unwrap();
                }
            }
        }

        icon.style().set_property("opacity", "1").unwrap();
    }).forget();

    Ok(())
}
