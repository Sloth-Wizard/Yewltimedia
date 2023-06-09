use yew::prelude::*;
use yew::{Component, Context};
use web_sys::HtmlElement;

use wasm_bindgen::prelude::*;

use crate::models::cards::*;
use crate::tools::constants::*;
use crate::tools::card_actions::*;

// Enumerate all the methods that should be available to the component here
pub enum Msg {
    PlaySprite(NodeRef),
    ShowMore(NodeRef, NodeRef, NodeRef),
    InjectDescription(HtmlElement)
}

pub struct CardComponent {
    card_component: CardComponentProps,
    card_ref: NodeRef,
    sprite_ref: NodeRef,
    button_ref: NodeRef,
    icon_ref: NodeRef
}

impl Component for CardComponent {
    type Message = Msg;
    type Properties = CardComponentProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            card_component: ctx.props().to_owned(),
            card_ref: NodeRef::default(),
            sprite_ref: NodeRef::default(),
            button_ref: NodeRef::default(),
            icon_ref: NodeRef::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // Card actions
            Msg::PlaySprite(sprite) => { sprite_animation(sprite.cast::<HtmlElement>(), ANIM_MS) }
            Msg::ShowMore(button, icon, card) => {
                match show_more(
                    button.cast::<HtmlElement>(), 
                    icon.cast::<HtmlElement>(), 
                    card.cast::<HtmlElement>(), 
                    self.card_component.description_container.cast::<HtmlElement>()
                ) {
                    OpenOrClose::Open(cards, card) => {
                        // Open the card then display the description
                        // Description should be show after the selected animation time
                        match open_card(cards, card, ANIM_MS) {
                            Ok(button) => {
                                ctx.link().send_future(async {
                                    Msg::InjectDescription(button)
                                })
                            },
                            Err(err) => {
                                log_err(err.as_string().unwrap().as_str());
                                return false
                            }
                        };
                        true
                    },
                    OpenOrClose::Close(cards, card) => {
                        // Hide the description, then close the card
                        // Card should be closed after the selected animation time
                        if let Some(container) = ctx.props().description_container.clone().cast::<HtmlElement>() {
                            match close_card(cards, card, container, ANIM_MS) {
                                Ok(button) => {
                                    log(&button);
                                    return true
                                },
                                Err(err) => {
                                    log_err(err.as_string().unwrap().as_str());
                                    return false
                                }
                            }
                        }
                        false
                    },
                    OpenOrClose::IsPlaying => { true },
                    OpenOrClose::Error(err) => {
                        log_err(err);
                        false
                    }
                }
            },
            Msg::InjectDescription(button) => {
                let container = ctx.props().description_container.clone().cast::<HtmlElement>();
                if let Some(container) = container {
                    container.set_inner_html(
                        format!("
                            <div class=\"card__desc\">
                                <div class=\"card__desc--pos\">
                                    <div class=\"inner\">
                                        <img
                                            width=\"150\"
                                            height=\"150\"
                                            src={PRELOAD_DESC_IMAGE}
                                            data-src=\"./public/{}\"
                                            alt=\"{}\"
                                            class=\"js-lazy\"
                                        />
                                    </div>
                                    <h2>{}</h2>
                                    <div class=\"content\">
                                        {}
                                    </div>
                                </div>
                            </div>
                        ",
                            ctx.props().card.picture.clone(), 
                            ctx.props().card.name.clone(), 
                            ctx.props().card.name.clone(), 
                            ctx.props().card.description.clone()
                        ).as_str()
                    );
                    
                    load_description(button, container, ANIM_MS);
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the kData
        let card_data = self.card_component.card.clone();

        // Setup the HTMLElement's we will need
        let card = self.card_ref.clone();
        let sprite = self.sprite_ref.clone();
        let button = self.button_ref.clone();
        let icon = self.icon_ref.clone();

        html! {
            <>
                <card ref={card.clone()} class={classes!(String::from("card"))}>
                    <div ref={sprite.clone()} onmousedown={ctx.link().callback(move |_| Msg::PlaySprite(sprite.clone()))} class={classes!(String::from("card__img js-tap"))}>
                        <img class={classes!(String::from("js-lazy"))} src={PRELOAD_SPRITE_IMAGE} data-src={format!("./public/{}", card_data.sprite)} alt={card_data.name.to_owned()} draggable="false"/>
                    </div>
                    <div class={classes!(String::from("card__actions"))}>
                        <div ref={button.clone()} onmousedown={ctx.link().callback(move |_| Msg::ShowMore(button.clone(), icon.clone(), card.clone()))} class={classes!(String::from("more js-tap"))}>
                            <div ref={self.icon_ref.clone()} class={classes!(String::from("icon"))}>
                                <span></span>
                                <span></span>
                            </div>
                        </div>
                    </div>
                </card>
            </>
        }
    }
}

#[wasm_bindgen]
extern "C" {
    // Log MouseEvent
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &HtmlElement);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str, c: &str, d: &str);

    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn log_err(s: &str);
}
