use yew::prelude::*;
use yew::{Component, Context};
use web_sys::HtmlElement;

use crate::models::imageloader::*;
use crate::tools::imageloader_actions::*;

pub enum LazyImage {
    Load(NodeRef, String)
}

pub struct ImageLoaderComponent {
    image: ImageProps,
    image_ref: NodeRef
}

impl Component for ImageLoaderComponent {
    type Message = LazyImage;
    type Properties = ImageProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self { 
            image: ctx.props().to_owned(),
            image_ref: NodeRef::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            LazyImage::Load(image, rs_src) => {
                match is_visible(image.cast::<HtmlElement>(), rs_src) {
                    // Make both arms after checking is the image is visible
                    _ => todo!()
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // Get the image data
        let image_data = self.image.clone();

        // Setup the HTMLElement's we will need
        let image = self.image_ref.clone();

        html! {
            <>
                <div class={classes!(String::from("rs-lazy"))}>
                    <img ref={image.clone()} src={image_data.src} rs-src={image_data.rs_src.clone()} alt={image_data.alt} onload={ctx.link().callback(move |_| LazyImage::Load(image.clone(), image_data.rs_src.clone()))} />
                </div>
            </>
        }
    }
}
